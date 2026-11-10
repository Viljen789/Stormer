use crate::{DynamicalSystem, RuntimeError, RuntimeValue, Trajectory, TrajectorySample};

/// One numerical stepping method included in the first runtime milestone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationMethod {
    /// First-order explicit Euler stepping.
    Euler,
    /// Classical fourth-order Runge-Kutta stepping.
    RungeKutta4,
}

fn validate_state<S: DynamicalSystem + ?Sized>(
    system: &S,
    state: &[f64],
) -> Result<usize, RuntimeError> {
    let expected = system.state_dimension();
    if expected == 0 {
        return Err(RuntimeError::EmptySystem);
    }
    if state.len() != expected {
        return Err(RuntimeError::StateDimensionMismatch {
            expected,
            actual: state.len(),
        });
    }
    for (index, &value) in state.iter().enumerate() {
        if !value.is_finite() {
            return Err(RuntimeError::NonFiniteValue {
                value: RuntimeValue::State { index },
            });
        }
    }
    Ok(expected)
}

fn validate_step_size(step_size: f64) -> Result<(), RuntimeError> {
    if !step_size.is_finite() {
        return Err(RuntimeError::NonFiniteValue {
            value: RuntimeValue::StepSize,
        });
    }
    if step_size <= 0.0 {
        return Err(RuntimeError::NonPositiveStepSize);
    }
    Ok(())
}

/// Evaluates and validates one derivative vector without mutating the input state.
pub fn evaluate_derivative<S: DynamicalSystem + ?Sized>(
    system: &S,
    time: f64,
    state: &[f64],
) -> Result<Vec<f64>, RuntimeError> {
    let expected = system.state_dimension();
    if expected == 0 {
        return Err(RuntimeError::EmptySystem);
    }
    if !time.is_finite() {
        return Err(RuntimeError::NonFiniteValue {
            value: RuntimeValue::Time,
        });
    }
    validate_state(system, state)?;

    let derivative = system.derivative(time, state)?;
    if derivative.len() != expected {
        return Err(RuntimeError::DerivativeDimensionMismatch {
            expected,
            actual: derivative.len(),
        });
    }
    for (index, &value) in derivative.iter().enumerate() {
        if !value.is_finite() {
            return Err(RuntimeError::NonFiniteValue {
                value: RuntimeValue::Derivative { index },
            });
        }
    }
    Ok(derivative)
}

/// Advances one state by one explicit Euler step.
pub fn euler_step<S: DynamicalSystem + ?Sized>(
    system: &S,
    time: f64,
    state: &[f64],
    step_size: f64,
) -> Result<Vec<f64>, RuntimeError> {
    validate_step_size(step_size)?;
    let derivative = evaluate_derivative(system, time, state)?;
    Ok(state
        .iter()
        .zip(derivative.iter())
        .map(|(&value, &slope)| value + step_size * slope)
        .collect())
}

/// Integrates one system over a requested interval with a selected milestone method.
///
/// When `(end_time - start_time)` is not an exact multiple of `step_size`, full steps of
/// `step_size` are taken until the final sub-interval, which uses one shorter clamped step so the
/// trajectory terminates at `end_time`.
pub fn integrate<S: DynamicalSystem + ?Sized>(
    system: &S,
    method: IntegrationMethod,
    initial_state: &[f64],
    start_time: f64,
    end_time: f64,
    step_size: f64,
) -> Result<Trajectory, RuntimeError> {
    if !start_time.is_finite() {
        return Err(RuntimeError::NonFiniteValue {
            value: RuntimeValue::StartTime,
        });
    }
    if !end_time.is_finite() {
        return Err(RuntimeError::NonFiniteValue {
            value: RuntimeValue::EndTime,
        });
    }
    validate_step_size(step_size)?;
    if end_time < start_time {
        return Err(RuntimeError::EndBeforeStart);
    }
    validate_state(system, initial_state)?;

    let mut samples = Vec::new();
    let mut current_time = start_time;
    let mut current_state = initial_state.to_vec();
    samples.push(TrajectorySample {
        time: current_time,
        state: current_state.clone(),
    });

    let time_scale = start_time.abs().max(end_time.abs()).max(step_size);
    let snap_tolerance = 16.0 * f64::EPSILON * time_scale;
    let mut step_index: usize = 0;

    while current_time < end_time {
        step_index += 1;
        let unclamped_next_time = start_time + (step_index as f64) * step_size;
        let is_final_step =
            unclamped_next_time >= end_time || (end_time - unclamped_next_time) <= snap_tolerance;
        let (next_time, dt) = if is_final_step {
            (end_time, end_time - current_time)
        } else {
            (unclamped_next_time, unclamped_next_time - current_time)
        };

        if dt <= 0.0 || next_time <= current_time {
            return Err(RuntimeError::NonPositiveStepSize);
        }

        current_state = match method {
            IntegrationMethod::Euler => euler_step(system, current_time, &current_state, dt)?,
            IntegrationMethod::RungeKutta4 => rk4_step(system, current_time, &current_state, dt)?,
        };
        current_time = next_time;
        samples.push(TrajectorySample {
            time: current_time,
            state: current_state.clone(),
        });
    }

    Ok(Trajectory { samples })
}

/// Advances one state by one classical fourth-order Runge-Kutta step.
pub fn rk4_step<S: DynamicalSystem + ?Sized>(
    system: &S,
    time: f64,
    state: &[f64],
    step_size: f64,
) -> Result<Vec<f64>, RuntimeError> {
    validate_step_size(step_size)?;
    let half_step = 0.5 * step_size;
    let midpoint_time = time + half_step;

    let k1 = evaluate_derivative(system, time, state)?;

    let state_k2: Vec<f64> = state
        .iter()
        .zip(k1.iter())
        .map(|(&y, &d1)| y + half_step * d1)
        .collect();
    let k2 = evaluate_derivative(system, midpoint_time, &state_k2)?;

    let state_k3: Vec<f64> = state
        .iter()
        .zip(k2.iter())
        .map(|(&y, &d2)| y + half_step * d2)
        .collect();
    let k3 = evaluate_derivative(system, midpoint_time, &state_k3)?;

    let state_k4: Vec<f64> = state
        .iter()
        .zip(k3.iter())
        .map(|(&y, &d3)| y + step_size * d3)
        .collect();
    let k4 = evaluate_derivative(system, time + step_size, &state_k4)?;

    let sixth_step = step_size / 6.0;
    Ok((0..state.len())
        .map(|i| state[i] + sixth_step * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]))
        .collect())
}
