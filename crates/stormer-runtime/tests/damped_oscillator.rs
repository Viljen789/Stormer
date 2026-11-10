use stormer_runtime::{DynamicalSystem, IntegrationMethod, RuntimeError, integrate};

struct DampedOscillator {
    mass: f64,
    damping: f64,
    stiffness: f64,
}

impl DynamicalSystem for DampedOscillator {
    fn state_dimension(&self) -> usize {
        2
    }

    fn derivative(&self, _time: f64, state: &[f64]) -> Result<Vec<f64>, RuntimeError> {
        if state.len() != self.state_dimension() {
            return Err(RuntimeError::StateDimensionMismatch {
                expected: self.state_dimension(),
                actual: state.len(),
            });
        }
        let position = state[0];
        let velocity = state[1];
        let acceleration =
            -(self.damping / self.mass) * velocity - (self.stiffness / self.mass) * position;
        Ok(vec![velocity, acceleration])
    }
}

fn mechanical_energy(system: &DampedOscillator, state: &[f64]) -> f64 {
    let position = state[0];
    let velocity = state[1];

    0.5 * system.mass * velocity * velocity + 0.5 * system.stiffness * position * position
}

fn exact_underdamped_position(system: &DampedOscillator, initial_position: f64, time: f64) -> f64 {
    let gamma = system.damping / (2.0 * system.mass);
    let omega_0_sq = system.stiffness / system.mass;
    let omega_d = (omega_0_sq - gamma * gamma).sqrt();
    let decay = (-gamma * time).exp();
    initial_position * decay * ((omega_d * time).cos() + (gamma / omega_d) * (omega_d * time).sin())
}

#[test]
fn damped_oscillator_uses_position_then_velocity() {
    let system = DampedOscillator {
        mass: 1.0,
        damping: 0.5,
        stiffness: 4.0,
    };

    let derivative = system
        .derivative(0.0, &[1.0, 0.0])
        .expect("the canonical initial state should be valid");

    assert_eq!(derivative, vec![0.0, -4.0]);
}

#[test]
fn positive_damping_reduces_mechanical_energy() {
    let system = DampedOscillator {
        mass: 1.0,
        damping: 0.5,
        stiffness: 4.0,
    };
    let initial_state = [1.0, 0.0];

    let trajectory = integrate(
        &system,
        IntegrationMethod::RungeKutta4,
        &initial_state,
        0.0,
        5.0,
        0.01,
    )
    .expect("the canonical simulation should be valid");
    let final_state = trajectory
        .samples()
        .last()
        .expect("the trajectory should contain samples")
        .state();

    assert!(mechanical_energy(&system, final_state) < mechanical_energy(&system, &initial_state));
}

#[test]
fn undamped_oscillator_preserves_mechanical_energy_with_rk4() {
    // System: m = 1.0, c = 0.0, k = 4.0 (natural frequency omega_0 = 2.0 rad/s).
    // Interval: [0.0, 5.0] with h = 0.01 using classical RK4.
    // Expected: exact mechanical energy is conserved at E = 2.0; RK4 drift over [0, 5] is < 1e-7.
    let system = DampedOscillator {
        mass: 1.0,
        damping: 0.0,
        stiffness: 4.0,
    };
    let initial_state = [1.0, 0.0];
    let initial_energy = mechanical_energy(&system, &initial_state);

    let trajectory = integrate(
        &system,
        IntegrationMethod::RungeKutta4,
        &initial_state,
        0.0,
        5.0,
        0.01,
    )
    .expect("the undamped simulation should be valid");
    let final_state = trajectory
        .samples()
        .last()
        .expect("the trajectory should contain samples")
        .state();

    let final_energy = mechanical_energy(&system, final_state);
    assert!((final_energy - initial_energy).abs() < 1.0e-7);
}

#[test]
fn excessive_euler_step_causes_numerical_instability() {
    // System: m = 1.0, c = 0.1, k = 4.0 (positively damped, so true physical energy decays).
    // Interval: [0.0, 10.0] with an intentionally excessive Euler step h = 0.5.
    // Expected: explicit Euler's amplification factor exceeds 1, injecting artificial energy.
    let system = DampedOscillator {
        mass: 1.0,
        damping: 0.1,
        stiffness: 4.0,
    };
    let initial_state = [1.0, 0.0];
    let initial_energy = mechanical_energy(&system, &initial_state);

    let trajectory = integrate(
        &system,
        IntegrationMethod::Euler,
        &initial_state,
        0.0,
        10.0,
        0.5,
    )
    .expect("large-step Euler remains finite over [0, 10] while blowing up in amplitude");
    let final_state = trajectory
        .samples()
        .last()
        .expect("the trajectory should contain samples")
        .state();

    assert!(mechanical_energy(&system, final_state) > 100.0 * initial_energy);
}

#[test]
fn rk4_is_more_accurate_than_euler_and_exhibits_expected_convergence() {
    // System: m = 1.0, c = 0.4, k = 4.0, x(0) = 1.0, v(0) = 0.0 over [0.0, 2.0].
    // Compare simulated position x(2.0) against exact underdamped analytical solution at h = 0.02 and h = 0.01.
    let system = DampedOscillator {
        mass: 1.0,
        damping: 0.4,
        stiffness: 4.0,
    };
    let initial_state = [1.0, 0.0];
    let end_time = 2.0;
    let exact_x = exact_underdamped_position(&system, initial_state[0], end_time);

    let euler_coarse = integrate(
        &system,
        IntegrationMethod::Euler,
        &initial_state,
        0.0,
        end_time,
        0.02,
    )
    .expect("coarse Euler should succeed");
    let euler_fine = integrate(
        &system,
        IntegrationMethod::Euler,
        &initial_state,
        0.0,
        end_time,
        0.01,
    )
    .expect("fine Euler should succeed");

    let rk4_coarse = integrate(
        &system,
        IntegrationMethod::RungeKutta4,
        &initial_state,
        0.0,
        end_time,
        0.02,
    )
    .expect("coarse RK4 should succeed");
    let rk4_fine = integrate(
        &system,
        IntegrationMethod::RungeKutta4,
        &initial_state,
        0.0,
        end_time,
        0.01,
    )
    .expect("fine RK4 should succeed");

    let euler_err_coarse = (euler_coarse.samples().last().unwrap().state()[0] - exact_x).abs();
    let euler_err_fine = (euler_fine.samples().last().unwrap().state()[0] - exact_x).abs();
    let rk4_err_coarse = (rk4_coarse.samples().last().unwrap().state()[0] - exact_x).abs();
    let rk4_err_fine = (rk4_fine.samples().last().unwrap().state()[0] - exact_x).abs();

    // RK4 at h = 0.02 is orders of magnitude more accurate than Euler at h = 0.02.
    assert!(rk4_err_coarse < 1.0e-5 * euler_err_coarse);
    // First-order Euler halves its global error when step size is halved.
    assert!(euler_err_fine < 0.6 * euler_err_coarse);
    // Fourth-order RK4 reduces its global error by roughly 2^4 = 16x when step size is halved.
    assert!(rk4_err_fine < 0.1 * rk4_err_coarse);
}
