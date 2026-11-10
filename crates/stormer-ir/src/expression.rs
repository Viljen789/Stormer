use crate::{ExprId, ModelError, ParameterId, StateId};

/// One node in a scalar equation expression graph.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// A dimensionless numeric constant.
    Constant(f64),
    /// A reference to a declared model parameter.
    Parameter(ParameterId),
    /// A reference to a declared state variable.
    State(StateId),
    /// Unary negation of one expression.
    Negate { operand: ExprId },
    /// Addition of two expressions.
    Add { left: ExprId, right: ExprId },
    /// Subtraction of two expressions.
    Subtract { left: ExprId, right: ExprId },
    /// Multiplication of two expressions.
    Multiply { left: ExprId, right: ExprId },
    /// Division of two expressions.
    Divide { left: ExprId, right: ExprId },
}

/// Owned storage for expression nodes addressed by typed stable indices.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExpressionArena {
    nodes: Vec<Expression>,
}

impl ExpressionArena {
    /// Creates an empty expression arena.
    #[must_use]
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Returns the number of stored expression nodes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Reports whether the arena stores no expression nodes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Stores an expression node and returns its stable typed identifier.
    pub fn insert(&mut self, expression: Expression) -> ExprId {
        let id = ExprId::from_index(self.nodes.len());
        self.nodes.push(expression);
        id
    }

    /// Looks up one expression node without panicking on an invalid identifier.
    pub fn get(&self, id: ExprId) -> Result<&Expression, ModelError> {
        self.nodes
            .get(id.index())
            .ok_or(ModelError::InvalidExpressionId { id })
    }

    /// Checks that every expression operand refers to an earlier available arena node.
    ///
    /// Requiring `operand.index() < expression.index()` enforces topological insertion order and
    /// guarantees that the expression graph is acyclic.
    pub fn validate_operands(&self) -> Result<(), ModelError> {
        for (index, node) in self.nodes.iter().enumerate() {
            let expression = ExprId::from_index(index);
            let check_operand = |operand: ExprId| -> Result<(), ModelError> {
                if operand.index() < index {
                    Ok(())
                } else {
                    Err(ModelError::UnavailableExpressionOperand {
                        expression,
                        operand,
                    })
                }
            };

            match *node {
                Expression::Constant(_) | Expression::Parameter(_) | Expression::State(_) => {}
                Expression::Negate { operand } => {
                    check_operand(operand)?;
                }
                Expression::Add { left, right }
                | Expression::Subtract { left, right }
                | Expression::Multiply { left, right }
                | Expression::Divide { left, right } => {
                    check_operand(left)?;
                    check_operand(right)?;
                }
            }
        }
        Ok(())
    }
}
