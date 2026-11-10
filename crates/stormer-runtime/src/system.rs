use crate::RuntimeError;

/// A first-order ordinary differential-equation system with fixed parameters.
///
/// State entries have a stable system-defined order.
/// Implementations read time and state without mutating the caller's state.
/// The returned derivative vector must contain exactly [`Self::state_dimension`] finite values in
/// the same state order.
pub trait DynamicalSystem {
    /// Returns the number of scalar values in one complete runtime state.
    fn state_dimension(&self) -> usize;

    /// Evaluates the time derivative at one time and state.
    fn derivative(&self, time: f64, state: &[f64]) -> Result<Vec<f64>, RuntimeError>;
}
