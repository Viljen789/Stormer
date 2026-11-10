use stormer_runtime::{DynamicalSystem, RuntimeError, RuntimeValue, evaluate_derivative};

struct MustNotEvaluate {
    dimension: usize,
}

impl DynamicalSystem for MustNotEvaluate {
    fn state_dimension(&self) -> usize {
        self.dimension
    }

    fn derivative(&self, _time: f64, _state: &[f64]) -> Result<Vec<f64>, RuntimeError> {
        panic!("invalid inputs must be rejected before derivative evaluation")
    }
}

struct WrongDerivativeShape;

impl DynamicalSystem for WrongDerivativeShape {
    fn state_dimension(&self) -> usize {
        2
    }

    fn derivative(&self, _time: f64, _state: &[f64]) -> Result<Vec<f64>, RuntimeError> {
        Ok(vec![0.0])
    }
}

struct NonFiniteDerivative;

impl DynamicalSystem for NonFiniteDerivative {
    fn state_dimension(&self) -> usize {
        2
    }

    fn derivative(&self, _time: f64, _state: &[f64]) -> Result<Vec<f64>, RuntimeError> {
        Ok(vec![0.0, f64::INFINITY])
    }
}

#[test]
fn empty_system_is_reported_before_evaluation() {
    assert_eq!(
        evaluate_derivative(&MustNotEvaluate { dimension: 0 }, 0.0, &[]),
        Err(RuntimeError::EmptySystem)
    );
}

#[test]
fn wrong_state_dimension_is_reported() {
    assert_eq!(
        evaluate_derivative(&MustNotEvaluate { dimension: 2 }, 0.0, &[1.0]),
        Err(RuntimeError::StateDimensionMismatch {
            expected: 2,
            actual: 1,
        })
    );
}

#[test]
fn non_finite_time_is_reported() {
    assert_eq!(
        evaluate_derivative(&MustNotEvaluate { dimension: 2 }, f64::NAN, &[1.0, 0.0]),
        Err(RuntimeError::NonFiniteValue {
            value: RuntimeValue::Time,
        })
    );
}

#[test]
fn wrong_derivative_dimension_is_reported() {
    assert_eq!(
        evaluate_derivative(&WrongDerivativeShape, 0.0, &[1.0, 0.0]),
        Err(RuntimeError::DerivativeDimensionMismatch {
            expected: 2,
            actual: 1,
        })
    );
}

#[test]
fn non_finite_state_is_reported() {
    assert_eq!(
        evaluate_derivative(&MustNotEvaluate { dimension: 2 }, 0.0, &[1.0, f64::NAN],),
        Err(RuntimeError::NonFiniteValue {
            value: RuntimeValue::State { index: 1 },
        })
    );
}

#[test]
fn non_finite_derivative_is_reported() {
    assert_eq!(
        evaluate_derivative(&NonFiniteDerivative, 0.0, &[1.0, 0.0]),
        Err(RuntimeError::NonFiniteValue {
            value: RuntimeValue::Derivative { index: 1 },
        })
    );
}
