use stormer_runtime::{DynamicalSystem, RuntimeError, RuntimeValue, rk4_step};

struct ExponentialGrowth;

impl DynamicalSystem for ExponentialGrowth {
    fn state_dimension(&self) -> usize {
        1
    }

    fn derivative(&self, _time: f64, state: &[f64]) -> Result<Vec<f64>, RuntimeError> {
        Ok(vec![state[0]])
    }
}

#[test]
fn rk4_step_matches_a_linear_reference() {
    let next = rk4_step(&ExponentialGrowth, 0.0, &[1.0], 0.1).expect("the step should be valid");
    let expected = 0.1_f64.exp();

    assert!((next[0] - expected).abs() < 1.0e-6);
}

#[test]
fn halving_rk4_step_reduces_error() {
    let one_step =
        rk4_step(&ExponentialGrowth, 0.0, &[1.0], 0.2).expect("one larger step should be valid")[0];
    let first_half = rk4_step(&ExponentialGrowth, 0.0, &[1.0], 0.1)
        .expect("the first half step should be valid");
    let two_steps = rk4_step(&ExponentialGrowth, 0.1, &first_half, 0.1)
        .expect("the second half step should be valid")[0];
    let expected = 0.2_f64.exp();

    assert!((two_steps - expected).abs() < (one_step - expected).abs());
}

#[test]
fn rk4_step_does_not_mutate_its_input() {
    let state = vec![1.0];
    let original = state.clone();

    let _next = rk4_step(&ExponentialGrowth, 0.0, &state, 0.1).expect("the step should be valid");

    assert_eq!(state, original);
}

#[test]
fn rk4_step_rejects_a_non_positive_step_size() {
    assert_eq!(
        rk4_step(&ExponentialGrowth, 0.0, &[1.0], 0.0),
        Err(RuntimeError::NonPositiveStepSize)
    );
}

#[test]
fn rk4_step_rejects_a_non_finite_step_size() {
    assert_eq!(
        rk4_step(&ExponentialGrowth, 0.0, &[1.0], f64::NAN),
        Err(RuntimeError::NonFiniteValue {
            value: RuntimeValue::StepSize,
        })
    );
}
