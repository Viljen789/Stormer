use stormer_runtime::{DynamicalSystem, RuntimeError, RuntimeValue, euler_step};

struct ConstantDerivative {
    rate: f64,
}

impl DynamicalSystem for ConstantDerivative {
    fn state_dimension(&self) -> usize {
        1
    }

    fn derivative(&self, _time: f64, _state: &[f64]) -> Result<Vec<f64>, RuntimeError> {
        Ok(vec![self.rate])
    }
}

#[test]
fn euler_step_matches_constant_derivative() {
    let system = ConstantDerivative { rate: 2.0 };

    let next = euler_step(&system, 0.0, &[1.0], 0.25).expect("the step should be valid");

    assert_eq!(next, vec![1.5]);
}

#[test]
fn euler_step_does_not_mutate_its_input() {
    let system = ConstantDerivative { rate: -0.5 };
    let state = vec![3.0];
    let original = state.clone();

    let _next = euler_step(&system, 1.0, &state, 0.1).expect("the step should be valid");

    assert_eq!(state, original);
}

#[test]
fn euler_step_rejects_a_non_positive_step_size() {
    let system = ConstantDerivative { rate: 1.0 };

    assert_eq!(
        euler_step(&system, 0.0, &[1.0], -0.1),
        Err(RuntimeError::NonPositiveStepSize)
    );
}

#[test]
fn euler_step_rejects_a_non_finite_step_size() {
    let system = ConstantDerivative { rate: 1.0 };

    assert_eq!(
        euler_step(&system, 0.0, &[1.0], f64::INFINITY),
        Err(RuntimeError::NonFiniteValue {
            value: RuntimeValue::StepSize,
        })
    );
}
