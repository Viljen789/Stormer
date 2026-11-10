/// One numerical input category that must contain only finite values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeValue {
    /// The time supplied to one derivative or step evaluation.
    Time,
    /// The beginning of a requested simulation interval.
    StartTime,
    /// The end of a requested simulation interval.
    EndTime,
    /// The requested integration step size.
    StepSize,
    /// One entry in a runtime state vector.
    State { index: usize },
    /// One entry returned by derivative evaluation.
    Derivative { index: usize },
}

/// A structural or numerical precondition failure in the runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeError {
    /// A system declared no runtime state slots.
    EmptySystem,
    /// A supplied state vector does not match the system's declared dimension.
    StateDimensionMismatch { expected: usize, actual: usize },
    /// A derivative vector does not match the system's declared dimension.
    DerivativeDimensionMismatch { expected: usize, actual: usize },
    /// A value that participates in numerical execution is NaN or infinite.
    NonFiniteValue { value: RuntimeValue },
    /// An integration step must advance time by a positive amount.
    NonPositiveStepSize,
    /// A simulation interval cannot end before it starts.
    EndBeforeStart,
}
