use crate::{
    BaseDimension, Dimension, DimensionError, ExprId, Expression, ExpressionArena, ModelError,
    ParameterId, StateId,
};

/// A named model parameter with a physical dimension and initial numeric value.
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    name: String,
    dimension: Dimension,
    initial_value: f64,
}

impl Parameter {
    /// Creates a parameter declaration without allocating it into a model.
    #[must_use]
    pub fn new(name: String, dimension: Dimension, initial_value: f64) -> Self {
        Self {
            name,
            dimension,
            initial_value,
        }
    }

    /// Returns the declared parameter name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the declared physical dimension.
    #[must_use]
    pub fn dimension(&self) -> Dimension {
        self.dimension
    }

    /// Returns the initial numeric value.
    #[must_use]
    pub fn initial_value(&self) -> f64 {
        self.initial_value
    }
}

/// A named state variable with a physical dimension and initial numeric value.
#[derive(Debug, Clone, PartialEq)]
pub struct State {
    name: String,
    dimension: Dimension,
    initial_value: f64,
}

impl State {
    /// Creates a state declaration without allocating it into a model.
    #[must_use]
    pub fn new(name: String, dimension: Dimension, initial_value: f64) -> Self {
        Self {
            name,
            dimension,
            initial_value,
        }
    }

    /// Returns the declared state name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the declared physical dimension.
    #[must_use]
    pub fn dimension(&self) -> Dimension {
        self.dimension
    }

    /// Returns the initial numeric value.
    #[must_use]
    pub fn initial_value(&self) -> f64 {
        self.initial_value
    }
}

/// An equation that associates one state with the root of its derivative expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DerivativeEquation {
    state: StateId,
    expression: ExprId,
}

impl DerivativeEquation {
    /// Creates a derivative association after its identifiers have been checked.
    #[must_use]
    pub fn new(state: StateId, expression: ExprId) -> Self {
        Self { state, expression }
    }

    /// Returns the state whose derivative this equation defines.
    #[must_use]
    pub fn state(&self) -> StateId {
        self.state
    }

    /// Returns the root expression for the derivative.
    #[must_use]
    pub fn expression(&self) -> ExprId {
        self.expression
    }
}

/// An immutable, structurally validated model representation.
#[derive(Debug, Clone, PartialEq)]
pub struct Model {
    parameters: Vec<Parameter>,
    states: Vec<State>,
    expressions: ExpressionArena,
    derivatives: Vec<DerivativeEquation>,
}

impl Model {
    /// Returns parameter declarations in their preserved declaration order.
    #[must_use]
    pub fn parameters(&self) -> &[Parameter] {
        &self.parameters
    }

    /// Returns state declarations in their preserved declaration order.
    #[must_use]
    pub fn states(&self) -> &[State] {
        &self.states
    }

    /// Returns the model's expression arena.
    #[must_use]
    pub fn expressions(&self) -> &ExpressionArena {
        &self.expressions
    }

    /// Returns derivative equations in their preserved construction order.
    #[must_use]
    pub fn derivatives(&self) -> &[DerivativeEquation] {
        &self.derivatives
    }

    /// Looks up a parameter declaration by typed identifier.
    pub fn parameter(&self, id: ParameterId) -> Result<&Parameter, ModelError> {
        self.parameters
            .get(id.index())
            .ok_or(ModelError::InvalidParameterId { id })
    }

    /// Looks up a state declaration by typed identifier.
    pub fn state(&self, id: StateId) -> Result<&State, ModelError> {
        self.states
            .get(id.index())
            .ok_or(ModelError::InvalidStateId { id })
    }

    /// Infers the physical dimension of one expression in the model's arena.
    pub fn infer_expression_dimension(&self, id: ExprId) -> Result<Dimension, DimensionError> {
        let node = self.expressions.get(id)?;
        match *node {
            Expression::Constant(_) => Ok(Dimension::dimensionless()),
            Expression::Parameter(param_id) => Ok(self.parameter(param_id)?.dimension()),
            Expression::State(state_id) => Ok(self.state(state_id)?.dimension()),
            Expression::Negate { operand } => self.infer_expression_dimension(operand),
            Expression::Add { left, right } | Expression::Subtract { left, right } => {
                let left_dim = self.infer_expression_dimension(left)?;
                let right_dim = self.infer_expression_dimension(right)?;
                if !left_dim.is_compatible_with(right_dim) {
                    return Err(DimensionError::IncompatibleOperands {
                        expression: id,
                        left: left_dim,
                        right: right_dim,
                    });
                }
                Ok(left_dim)
            }
            Expression::Multiply { left, right } => {
                let left_dim = self.infer_expression_dimension(left)?;
                let right_dim = self.infer_expression_dimension(right)?;
                Ok(left_dim.multiply(right_dim))
            }
            Expression::Divide { left, right } => {
                let left_dim = self.infer_expression_dimension(left)?;
                let right_dim = self.infer_expression_dimension(right)?;
                Ok(left_dim.divide(right_dim))
            }
        }
    }

    /// Validates that all expressions in the arena are dimensionally consistent and that every
    /// state derivative equation has dimension `state.dimension() / Time`.
    pub fn validate_dimensions(&self) -> Result<(), DimensionError> {
        for index in 0..self.expressions.len() {
            self.infer_expression_dimension(ExprId::from_index(index))?;
        }
        let time_dim = Dimension::base(BaseDimension::Time);
        for derivative in &self.derivatives {
            let state = self.state(derivative.state())?;
            let expected = state.dimension().divide(time_dim);
            let actual = self.infer_expression_dimension(derivative.expression())?;
            if !actual.is_compatible_with(expected) {
                return Err(DimensionError::DerivativeDimensionMismatch {
                    state: derivative.state(),
                    expected,
                    actual,
                });
            }
        }
        Ok(())
    }
}

/// Mutable construction boundary for producing an immutable valid [`Model`].
#[derive(Debug, Default)]
pub struct ModelBuilder {
    parameters: Vec<Parameter>,
    states: Vec<State>,
    expressions: ExpressionArena,
    derivatives: Vec<DerivativeEquation>,
}

impl ModelBuilder {
    /// Creates an empty model builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            parameters: Vec::new(),
            states: Vec::new(),
            expressions: ExpressionArena::new(),
            derivatives: Vec::new(),
        }
    }

    /// Adds a parameter declaration and returns its stable identifier.
    pub fn add_parameter(
        &mut self,
        name: String,
        dimension: Dimension,
        initial_value: f64,
    ) -> ParameterId {
        let id = ParameterId::from_index(self.parameters.len());
        self.parameters
            .push(Parameter::new(name, dimension, initial_value));
        id
    }

    /// Adds a state declaration and returns its stable identifier.
    pub fn add_state(&mut self, name: String, dimension: Dimension, initial_value: f64) -> StateId {
        let id = StateId::from_index(self.states.len());
        self.states.push(State::new(name, dimension, initial_value));
        id
    }

    /// Adds an expression node to the builder-owned arena.
    pub fn insert_expression(&mut self, expression: Expression) -> ExprId {
        self.expressions.insert(expression)
    }

    /// Assigns one derivative expression to a state after structural checks.
    pub fn assign_derivative(
        &mut self,
        state: StateId,
        expression: ExprId,
    ) -> Result<(), ModelError> {
        if state.index() >= self.states.len() {
            return Err(ModelError::UnavailableDerivativeTarget { state });
        }
        if self
            .derivatives
            .iter()
            .any(|equation| equation.state() == state)
        {
            return Err(ModelError::DuplicateDerivativeEquation { state });
        }
        self.derivatives
            .push(DerivativeEquation::new(state, expression));
        Ok(())
    }

    /// Checks required completeness and converts the builder into an immutable model.
    pub fn finish(self) -> Result<Model, ModelError> {
        for index in 0..self.states.len() {
            let state = StateId::from_index(index);
            if !self
                .derivatives
                .iter()
                .any(|equation| equation.state() == state)
            {
                return Err(ModelError::MissingDerivativeEquation { state });
            }
        }

        self.expressions.validate_operands()?;

        for index in 0..self.expressions.len() {
            let expr_id = ExprId::from_index(index);
            match *self.expressions.get(expr_id)? {
                Expression::Parameter(id) if id.index() >= self.parameters.len() => {
                    return Err(ModelError::InvalidParameterId { id });
                }
                Expression::State(id) if id.index() >= self.states.len() => {
                    return Err(ModelError::InvalidStateId { id });
                }
                _ => {}
            }
        }

        for equation in &self.derivatives {
            if equation.state().index() >= self.states.len() {
                return Err(ModelError::InvalidStateId {
                    id: equation.state(),
                });
            }
            self.expressions.get(equation.expression())?;
        }

        Ok(Model {
            parameters: self.parameters,
            states: self.states,
            expressions: self.expressions,
            derivatives: self.derivatives,
        })
    }
}
