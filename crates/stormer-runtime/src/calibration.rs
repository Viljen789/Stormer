use stormer_ir::{Model, ParameterId, StateId};

use crate::{IntegrationMethod, IrSystem, RuntimeError, RuntimeValue, integrate};

/// A single timestamped scalar measurement of a modeled state variable.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Observation {
    /// Observation timestamp.
    pub time: f64,
    /// Measured scalar value of the observed state.
    pub value: f64,
}

impl Observation {
    /// Creates a single timestamped observation.
    #[must_use]
    pub const fn new(time: f64, value: f64) -> Self {
        Self { time, value }
    }
}

/// Configuration for numerical parameter calibration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CalibrationConfig {
    /// Numerical integration method used during forward trajectory evaluations.
    pub method: IntegrationMethod,
    /// Maximum integration step size within each observation interval.
    pub step_size: f64,
    /// Maximum number of optimization iterations.
    pub max_iterations: usize,
    /// Stopping threshold on the absolute loss gradient $|d\mathcal{L}/d\theta|$.
    pub gradient_tolerance: f64,
    /// Stopping threshold on the parameter update magnitude $|\Delta\theta|$.
    pub step_tolerance: f64,
}

impl Default for CalibrationConfig {
    fn default() -> Self {
        Self {
            method: IntegrationMethod::RungeKutta4,
            step_size: 0.01,
            max_iterations: 50,
            gradient_tolerance: 1.0e-10,
            step_tolerance: 1.0e-10,
        }
    }
}

/// Result and convergence evidence from fitting a physical model parameter to observations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CalibrationResult {
    /// Calibrated parameter identifier.
    pub parameter: ParameterId,
    /// Initial parameter guess supplied to the optimizer.
    pub initial_guess: f64,
    /// Final estimated parameter value.
    pub estimated_value: f64,
    /// Mean-squared error loss at `initial_guess`.
    pub initial_loss: f64,
    /// Mean-squared error loss at `estimated_value`.
    pub final_loss: f64,
    /// Central finite-difference loss derivative $d\mathcal{L}/d\theta$ at `estimated_value`.
    pub final_sensitivity: f64,
    /// Number of optimization iterations performed.
    pub iterations: usize,
    /// Whether the optimizer satisfied its gradient or step convergence criterion.
    pub converged: bool,
}

/// Simulates `system` from `t = 0.0` across all sorted `observations` and returns the residual
/// vector $r_i = \hat{x}(t_i) - y_i$.
pub fn evaluate_residuals(
    system: &IrSystem<'_>,
    observable_state: StateId,
    observations: &[Observation],
    method: IntegrationMethod,
    step_size: f64,
) -> Result<Vec<f64>, RuntimeError> {
    if observations.is_empty() {
        return Err(RuntimeError::EmptySystem);
    }
    if observable_state.index() >= system.model().states().len() {
        return Err(RuntimeError::StateDimensionMismatch {
            expected: system.model().states().len(),
            actual: observable_state.index(),
        });
    }

    let mut current_time = 0.0;
    let mut current_state = system.initial_state();
    let mut residuals = Vec::with_capacity(observations.len());

    for obs in observations {
        if !obs.time.is_finite() {
            return Err(RuntimeError::NonFiniteValue {
                value: RuntimeValue::Time,
            });
        }
        if !obs.value.is_finite() {
            return Err(RuntimeError::NonFiniteValue {
                value: RuntimeValue::State {
                    index: observable_state.index(),
                },
            });
        }
        if obs.time < current_time {
            return Err(RuntimeError::EndBeforeStart);
        }

        if obs.time > current_time {
            let segment = integrate(
                system,
                method,
                &current_state,
                current_time,
                obs.time,
                step_size,
            )?;
            if let Some(last) = segment.samples().last() {
                current_time = last.time();
                current_state = last.state().to_vec();
            }
        }

        let simulated = current_state[observable_state.index()];
        residuals.push(simulated - obs.value);
    }

    Ok(residuals)
}

/// Computes the Mean Squared Error (MSE) loss between simulated state values and observations:
/// $\mathcal{L}(\theta) = \frac{1}{N} \sum_{i=1}^N (\hat{x}(t_i; \theta) - y_i)^2$.
pub fn evaluate_mse_loss(
    system: &IrSystem<'_>,
    observable_state: StateId,
    observations: &[Observation],
    method: IntegrationMethod,
    step_size: f64,
) -> Result<f64, RuntimeError> {
    let residuals = evaluate_residuals(system, observable_state, observations, method, step_size)?;
    let sum_sq: f64 = residuals.iter().map(|r| r * r).sum();
    Ok(sum_sq / (residuals.len() as f64))
}

/// Computes the central finite-difference sensitivity $d\mathcal{L}/d\theta$ of the MSE loss with
/// respect to `parameter`.
pub fn finite_difference_sensitivity(
    system: &IrSystem<'_>,
    parameter: ParameterId,
    observable_state: StateId,
    observations: &[Observation],
    method: IntegrationMethod,
    step_size: f64,
) -> Result<f64, RuntimeError> {
    let base_val = *system.parameter_values().get(parameter.index()).ok_or(
        RuntimeError::StateDimensionMismatch {
            expected: system.parameter_values().len(),
            actual: parameter.index(),
        },
    )?;
    let delta = 1.0e-6 * base_val.abs().max(1.0);

    let mut plus_sys = system.clone();
    plus_sys.set_parameter(parameter, base_val + delta)?;
    let loss_plus =
        evaluate_mse_loss(&plus_sys, observable_state, observations, method, step_size)?;

    let mut minus_sys = system.clone();
    minus_sys.set_parameter(parameter, base_val - delta)?;
    let loss_minus = evaluate_mse_loss(
        &minus_sys,
        observable_state,
        observations,
        method,
        step_size,
    )?;

    Ok((loss_plus - loss_minus) / (2.0 * delta))
}

/// Calibrates `parameter` against `observations` of `observable_state` using damped Gauss-Newton
/// (Levenberg-Marquardt) optimization with finite-difference residual sensitivities.
pub fn calibrate_parameter(
    model: &Model,
    parameter: ParameterId,
    initial_guess: f64,
    observable_state: StateId,
    observations: &[Observation],
    config: CalibrationConfig,
) -> Result<CalibrationResult, RuntimeError> {
    let mut system = IrSystem::new(model)?;
    system.set_parameter(parameter, initial_guess)?;

    let initial_residuals = evaluate_residuals(
        &system,
        observable_state,
        observations,
        config.method,
        config.step_size,
    )?;
    let n = initial_residuals.len() as f64;
    let initial_loss = initial_residuals.iter().map(|r| r * r).sum::<f64>() / n;

    let mut theta = initial_guess;
    let mut current_residuals = initial_residuals;
    let mut current_loss = initial_loss;
    let mut damping = 1.0e-6_f64;
    let mut converged = false;
    let mut iterations = 0;

    for iter in 0..config.max_iterations {
        let delta = 1.0e-6 * theta.abs().max(1.0);

        let mut plus_sys = system.clone();
        plus_sys.set_parameter(parameter, theta + delta)?;
        let res_plus = evaluate_residuals(
            &plus_sys,
            observable_state,
            observations,
            config.method,
            config.step_size,
        )?;

        let mut minus_sys = system.clone();
        minus_sys.set_parameter(parameter, theta - delta)?;
        let res_minus = evaluate_residuals(
            &minus_sys,
            observable_state,
            observations,
            config.method,
            config.step_size,
        )?;

        let mut grad = 0.0;
        let mut hess_approx = 0.0;
        for i in 0..current_residuals.len() {
            let j_i = (res_plus[i] - res_minus[i]) / (2.0 * delta);
            grad += (2.0 / n) * current_residuals[i] * j_i;
            hess_approx += (2.0 / n) * j_i * j_i;
        }

        if grad.abs() <= config.gradient_tolerance {
            converged = true;
            break;
        }

        iterations = iter + 1;
        let mut accepted = false;

        for _ in 0..12 {
            let step = -grad / (hess_approx + damping);
            if step.abs() <= config.step_tolerance {
                converged = true;
                accepted = true;
                break;
            }

            let candidate_theta = theta + step;
            let mut candidate_sys = system.clone();
            if candidate_sys
                .set_parameter(parameter, candidate_theta)
                .is_ok()
                && let Ok(candidate_residuals) = evaluate_residuals(
                    &candidate_sys,
                    observable_state,
                    observations,
                    config.method,
                    config.step_size,
                )
            {
                let candidate_loss = candidate_residuals.iter().map(|r| r * r).sum::<f64>() / n;
                if candidate_loss < current_loss {
                    theta = candidate_theta;
                    system = candidate_sys;
                    current_residuals = candidate_residuals;
                    current_loss = candidate_loss;
                    damping = (damping * 0.25).max(1.0e-12);
                    accepted = true;
                    if step.abs() <= config.step_tolerance {
                        converged = true;
                    }
                    break;
                }
            }
            damping *= 4.0;
        }

        if converged || !accepted {
            break;
        }
    }

    let final_sensitivity = finite_difference_sensitivity(
        &system,
        parameter,
        observable_state,
        observations,
        config.method,
        config.step_size,
    )?;

    Ok(CalibrationResult {
        parameter,
        initial_guess,
        estimated_value: theta,
        initial_loss,
        final_loss: current_loss,
        final_sensitivity,
        iterations,
        converged,
    })
}
