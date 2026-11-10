use stormer_runtime::{DynamicalSystem, IntegrationMethod, RuntimeError, RuntimeValue, integrate};

struct ConstantDerivative;

impl DynamicalSystem for ConstantDerivative {
    fn state_dimension(&self) -> usize {
        1
    }

    fn derivative(&self, _time: f64, _state: &[f64]) -> Result<Vec<f64>, RuntimeError> {
        Ok(vec![1.0])
    }
}

#[test]
fn trajectory_begins_with_the_initial_condition() {
    let initial_state = [3.5];

    let trajectory = integrate(
        &ConstantDerivative,
        IntegrationMethod::Euler,
        &initial_state,
        2.0,
        2.5,
        0.25,
    )
    .expect("the integration request should be valid");

    let first = trajectory
        .samples()
        .first()
        .expect("a valid trajectory should include its initial condition");
    assert_eq!(first.time().to_bits(), 2.0_f64.to_bits());
    assert_eq!(first.state(), initial_state);
}

#[test]
fn integration_rejects_a_non_positive_step_size() {
    assert_eq!(
        integrate(
            &ConstantDerivative,
            IntegrationMethod::Euler,
            &[0.0],
            0.0,
            1.0,
            0.0,
        ),
        Err(RuntimeError::NonPositiveStepSize)
    );
}

#[test]
fn integration_rejects_a_reversed_interval() {
    assert_eq!(
        integrate(
            &ConstantDerivative,
            IntegrationMethod::Euler,
            &[0.0],
            2.0,
            1.0,
            0.1,
        ),
        Err(RuntimeError::EndBeforeStart)
    );
}

#[test]
fn integration_rejects_a_non_finite_start_time() {
    assert_eq!(
        integrate(
            &ConstantDerivative,
            IntegrationMethod::Euler,
            &[0.0],
            f64::NAN,
            1.0,
            0.1,
        ),
        Err(RuntimeError::NonFiniteValue {
            value: RuntimeValue::StartTime,
        })
    );
}

#[test]
fn integration_rejects_a_non_finite_end_time() {
    assert_eq!(
        integrate(
            &ConstantDerivative,
            IntegrationMethod::Euler,
            &[0.0],
            0.0,
            f64::INFINITY,
            0.1,
        ),
        Err(RuntimeError::NonFiniteValue {
            value: RuntimeValue::EndTime,
        })
    );
}

#[test]
fn integration_rejects_a_non_finite_step_size() {
    assert_eq!(
        integrate(
            &ConstantDerivative,
            IntegrationMethod::Euler,
            &[0.0],
            0.0,
            1.0,
            f64::NAN,
        ),
        Err(RuntimeError::NonFiniteValue {
            value: RuntimeValue::StepSize,
        })
    );
}

#[test]
fn integration_validates_the_initial_state_before_stepping() {
    assert_eq!(
        integrate(
            &ConstantDerivative,
            IntegrationMethod::Euler,
            &[],
            0.0,
            0.0,
            0.1,
        ),
        Err(RuntimeError::StateDimensionMismatch {
            expected: 1,
            actual: 0,
        })
    );
}

#[test]
fn integration_rejects_a_non_finite_initial_state() {
    assert_eq!(
        integrate(
            &ConstantDerivative,
            IntegrationMethod::Euler,
            &[f64::NEG_INFINITY],
            0.0,
            0.0,
            0.1,
        ),
        Err(RuntimeError::NonFiniteValue {
            value: RuntimeValue::State { index: 0 },
        })
    );
}

#[test]
fn integration_clamps_final_step_for_non_divisible_interval() {
    let trajectory = integrate(
        &ConstantDerivative,
        IntegrationMethod::Euler,
        &[1.0],
        0.0,
        0.25,
        0.1,
    )
    .expect("non-divisible intervals should take a shorter clamped final step");

    let samples = trajectory.samples();
    assert_eq!(samples.len(), 4);
    assert_eq!(samples[0].time().to_bits(), 0.0_f64.to_bits());
    assert_eq!(samples[0].state(), &[1.0]);
    assert!((samples[1].time() - 0.1).abs() < 1.0e-15);
    assert!((samples[1].state()[0] - 1.1).abs() < 1.0e-15);
    assert!((samples[2].time() - 0.2).abs() < 1.0e-15);
    assert!((samples[2].state()[0] - 1.2).abs() < 1.0e-15);
    assert_eq!(samples[3].time().to_bits(), 0.25_f64.to_bits());
    assert!((samples[3].state()[0] - 1.25).abs() < 1.0e-15);
}
