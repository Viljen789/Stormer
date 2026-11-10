use crate::{Dimension, ExprId, ParameterId, StateId};

/// A structural failure encountered while looking up or constructing model IR.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelError {
    /// A parameter identifier does not refer to an available declaration.
    InvalidParameterId { id: ParameterId },
    /// A state identifier does not refer to an available declaration.
    InvalidStateId { id: StateId },
    /// An expression identifier does not refer to an available node.
    InvalidExpressionId { id: ExprId },
    /// An expression node refers to an operand that is unavailable in its arena.
    UnavailableExpressionOperand { expression: ExprId, operand: ExprId },
    /// A state already has a derivative equation.
    DuplicateDerivativeEquation { state: StateId },
    /// A state has no derivative equation when model construction finishes.
    MissingDerivativeEquation { state: StateId },
    /// A derivative equation targets a state that is unavailable in the builder.
    UnavailableDerivativeTarget { state: StateId },
}

/// A physical-dimension inconsistency encountered while checking expressions or derivative equations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DimensionError {
    /// Underlying structural lookup failed.
    Model(ModelError),
    /// An addition or subtraction combined operands with incompatible dimensions.
    IncompatibleOperands {
        /// The expression node where the mismatch occurred.
        expression: ExprId,
        /// The dimension of the left operand.
        left: Dimension,
        /// The dimension of the right operand.
        right: Dimension,
    },
    /// A derivative equation's expression does not match `state_dimension / Time`.
    DerivativeDimensionMismatch {
        /// The state whose derivative equation failed dimensional validation.
        state: StateId,
        /// Expected derivative dimension (`state_dimension / Time`).
        expected: Dimension,
        /// Actual inferred dimension of the derivative root expression.
        actual: Dimension,
    },
}

impl From<ModelError> for DimensionError {
    fn from(error: ModelError) -> Self {
        Self::Model(error)
    }
}
