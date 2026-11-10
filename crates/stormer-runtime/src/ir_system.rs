use stormer_ir::{ExprId, Expression, Model, ParameterId, StateId};

use crate::{
    DynamicalSystem, IntegrationMethod, RuntimeError, RuntimeValue, Trajectory, integrate,
};

/// Interpreted runtime adapter that executes a validated [`stormer_ir::Model`] without source-level
/// name lookup.
#[derive(Debug, Clone)]
pub struct IrSystem<'a> {
    model: &'a Model,
    parameters: Vec<f64>,
    derivative_roots: Vec<ExprId>,
}

impl<'a> IrSystem<'a> {
    /// Instantiates an executable runtime system from a validated [`Model`].
    pub fn new(model: &'a Model) -> Result<Self, RuntimeError> {
        let state_count = model.states().len();
        if state_count == 0 {
            return Err(RuntimeError::EmptySystem);
        }

        let mut parameters = Vec::with_capacity(model.parameters().len());
        for param in model.parameters() {
            let val = param.initial_value();
            if !val.is_finite() {
                return Err(RuntimeError::NonFiniteValue {
                    value: RuntimeValue::State { index: 0 },
                });
            }
            parameters.push(val);
        }

        let mut derivative_roots = vec![ExprId::from_index(0); state_count];
        for (slot, state_root) in derivative_roots.iter_mut().enumerate() {
            let target = StateId::from_index(slot);
            let equation = model
                .derivatives()
                .iter()
                .find(|eq| eq.state() == target)
                .ok_or(RuntimeError::StateDimensionMismatch {
                    expected: state_count,
                    actual: model.derivatives().len(),
                })?;
            *state_root = equation.expression();
        }

        Ok(Self {
            model,
            parameters,
            derivative_roots,
        })
    }

    /// Returns the underlying IR model.
    #[must_use]
    pub fn model(&self) -> &'a Model {
        self.model
    }

    /// Extracts the initial state vector in the model's canonical state slot order.
    #[must_use]
    pub fn initial_state(&self) -> Vec<f64> {
        self.model
            .states()
            .iter()
            .map(|state| state.initial_value())
            .collect()
    }

    /// Returns the current runtime parameter values.
    #[must_use]
    pub fn parameter_values(&self) -> &[f64] {
        &self.parameters
    }

    /// Updates a parameter value for simulation or calibration without mutating the IR model.
    pub fn set_parameter(&mut self, id: ParameterId, value: f64) -> Result<(), RuntimeError> {
        if !value.is_finite() {
            return Err(RuntimeError::NonFiniteValue {
                value: RuntimeValue::State { index: id.index() },
            });
        }
        let expected = self.parameters.len();
        let slot =
            self.parameters
                .get_mut(id.index())
                .ok_or(RuntimeError::StateDimensionMismatch {
                    expected,
                    actual: id.index(),
                })?;
        *slot = value;
        Ok(())
    }

    /// Resolves a parameter name to its [`ParameterId`] before simulation.
    #[must_use]
    pub fn find_parameter(&self, name: &str) -> Option<ParameterId> {
        self.model
            .parameters()
            .iter()
            .position(|param| param.name() == name)
            .map(ParameterId::from_index)
    }

    /// Resolves a state name to its [`StateId`] before simulation.
    #[must_use]
    pub fn find_state(&self, name: &str) -> Option<StateId> {
        self.model
            .states()
            .iter()
            .position(|state| state.name() == name)
            .map(StateId::from_index)
    }
}

impl DynamicalSystem for IrSystem<'_> {
    fn state_dimension(&self) -> usize {
        self.model.states().len()
    }

    fn derivative(&self, _time: f64, state: &[f64]) -> Result<Vec<f64>, RuntimeError> {
        let expected = self.state_dimension();
        if state.len() != expected {
            return Err(RuntimeError::StateDimensionMismatch {
                expected,
                actual: state.len(),
            });
        }

        let arena = self.model.expressions();
        let mut expr_values: Vec<f64> = Vec::with_capacity(arena.len());

        for index in 0..arena.len() {
            let node = arena
                .get(ExprId::from_index(index))
                .map_err(|_| RuntimeError::EmptySystem)?;
            let value = match *node {
                Expression::Constant(val) => val,
                Expression::Parameter(id) => self.parameters[id.index()],
                Expression::State(id) => state[id.index()],
                Expression::Negate { operand } => -expr_values[operand.index()],
                Expression::Add { left, right } => {
                    expr_values[left.index()] + expr_values[right.index()]
                }
                Expression::Subtract { left, right } => {
                    expr_values[left.index()] - expr_values[right.index()]
                }
                Expression::Multiply { left, right } => {
                    expr_values[left.index()] * expr_values[right.index()]
                }
                Expression::Divide { left, right } => {
                    expr_values[left.index()] / expr_values[right.index()]
                }
            };
            expr_values.push(value);
        }

        Ok(self
            .derivative_roots
            .iter()
            .map(|root| expr_values[root.index()])
            .collect())
    }
}

/// Instantiates and integrates a validated [`Model`] from its declared initial conditions.
pub fn simulate_model(
    model: &Model,
    method: IntegrationMethod,
    start_time: f64,
    end_time: f64,
    step_size: f64,
) -> Result<Trajectory, RuntimeError> {
    let system = IrSystem::new(model)?;
    let initial_state = system.initial_state();
    integrate(
        &system,
        method,
        &initial_state,
        start_time,
        end_time,
        step_size,
    )
}
