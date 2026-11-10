use stormer_ir::{BaseDimension, Dimension, Expression, Model, ModelBuilder};
use stormer_runtime::{
    CalibrationConfig, DynamicalSystem, IntegrationMethod, IrSystem, Observation, RuntimeError,
    calibrate_parameter, integrate, simulate_model,
};

struct HardCodedDampedOscillator {
    mass: f64,
    damping: f64,
    stiffness: f64,
}

impl DynamicalSystem for HardCodedDampedOscillator {
    fn state_dimension(&self) -> usize {
        2
    }

    fn derivative(&self, _time: f64, state: &[f64]) -> Result<Vec<f64>, RuntimeError> {
        let x = state[0];
        let v = state[1];
        Ok(vec![
            v,
            -(self.damping / self.mass) * v - (self.stiffness / self.mass) * x,
        ])
    }
}

fn build_damped_oscillator_ir(mass_val: f64, damping_val: f64, stiffness_val: f64) -> Model {
    let mut builder = ModelBuilder::new();
    let m_dim = Dimension::base(BaseDimension::Mass);
    let l_dim = Dimension::base(BaseDimension::Length);
    let t_dim = Dimension::base(BaseDimension::Time);

    let mass = builder.add_parameter("mass".to_owned(), m_dim, mass_val);
    let damping = builder.add_parameter("damping".to_owned(), m_dim.divide(t_dim), damping_val);
    let stiffness = builder.add_parameter(
        "stiffness".to_owned(),
        m_dim.divide(t_dim.pow(2)),
        stiffness_val,
    );

    let position = builder.add_state("position".to_owned(), l_dim, 1.0);
    let velocity = builder.add_state("velocity".to_owned(), l_dim.divide(t_dim), 0.0);

    let e_vel = builder.insert_expression(Expression::State(velocity));
    let e_pos = builder.insert_expression(Expression::State(position));
    let e_mass = builder.insert_expression(Expression::Parameter(mass));
    let e_damp = builder.insert_expression(Expression::Parameter(damping));
    let e_stiff = builder.insert_expression(Expression::Parameter(stiffness));

    let c_over_m = builder.insert_expression(Expression::Divide {
        left: e_damp,
        right: e_mass,
    });
    let neg_c_over_m = builder.insert_expression(Expression::Negate { operand: c_over_m });
    let damping_term = builder.insert_expression(Expression::Multiply {
        left: neg_c_over_m,
        right: e_vel,
    });

    let k_over_m = builder.insert_expression(Expression::Divide {
        left: e_stiff,
        right: e_mass,
    });
    let spring_term = builder.insert_expression(Expression::Multiply {
        left: k_over_m,
        right: e_pos,
    });

    let accel = builder.insert_expression(Expression::Subtract {
        left: damping_term,
        right: spring_term,
    });

    builder.assign_derivative(position, e_vel).unwrap();
    builder.assign_derivative(velocity, accel).unwrap();

    let model = builder.finish().unwrap();
    model.validate_dimensions().unwrap();
    model
}

#[test]
fn interpreted_ir_trajectory_matches_hard_coded_oscillator_exactly() {
    let model = build_damped_oscillator_ir(1.0, 0.5, 4.0);
    let ir_trajectory =
        simulate_model(&model, IntegrationMethod::RungeKutta4, 0.0, 5.0, 0.01).unwrap();

    let reference = HardCodedDampedOscillator {
        mass: 1.0,
        damping: 0.5,
        stiffness: 4.0,
    };
    let ref_trajectory = integrate(
        &reference,
        IntegrationMethod::RungeKutta4,
        &[1.0, 0.0],
        0.0,
        5.0,
        0.01,
    )
    .unwrap();

    assert_eq!(ir_trajectory, ref_trajectory);
}

#[test]
fn calibrates_unknown_damping_parameter_from_synthetic_position_observations() {
    let true_damping = 0.5;
    let true_model = build_damped_oscillator_ir(1.0, true_damping, 4.0);
    let true_trajectory =
        simulate_model(&true_model, IntegrationMethod::RungeKutta4, 0.0, 4.0, 0.01).unwrap();

    // Sample synthetic position observations every 0.2 seconds over [0.0, 4.0].
    let observations: Vec<Observation> = true_trajectory
        .samples()
        .iter()
        .step_by(20)
        .map(|s| Observation::new(s.time(), s.state()[0]))
        .collect();

    // Start from an inaccurate initial damping guess (c = 1.5 instead of 0.5).
    let model = build_damped_oscillator_ir(1.0, 1.5, 4.0);
    let sys = IrSystem::new(&model).unwrap();
    let damping_id = sys.find_parameter("damping").unwrap();
    let position_id = sys.find_state("position").unwrap();

    let result = calibrate_parameter(
        &model,
        damping_id,
        1.5,
        position_id,
        &observations,
        CalibrationConfig::default(),
    )
    .expect("calibration should succeed");

    assert!(result.converged);
    assert!(result.final_loss < 1.0e-12);
    assert!(result.final_loss < 1.0e-9 * result.initial_loss);
    assert!((result.estimated_value - true_damping).abs() < 1.0e-6);
}
