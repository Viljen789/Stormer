#![forbid(unsafe_code)]
//! Numerical execution, IR interpretation, and parameter calibration runtime for Størmer.

mod calibration;
mod error;
mod integration;
mod ir_system;
mod system;
mod trajectory;

pub use calibration::{
    CalibrationConfig, CalibrationResult, Observation, calibrate_parameter, evaluate_mse_loss,
    evaluate_residuals, finite_difference_sensitivity,
};
pub use error::{RuntimeError, RuntimeValue};
pub use integration::{IntegrationMethod, euler_step, evaluate_derivative, integrate, rk4_step};
pub use ir_system::{IrSystem, simulate_model};
pub use system::DynamicalSystem;
pub use trajectory::{Trajectory, TrajectorySample};
