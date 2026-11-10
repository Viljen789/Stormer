#![forbid(unsafe_code)]
//! Source-independent physical model intermediate representation for Størmer.
//!
//! This crate defines physical dimensions, typed identifiers, expression arenas, parameter and
//! state declarations, derivative equations, checked model construction, and dimensional
//! consistency verification.

mod dimension;
mod error;
mod expression;
mod ids;
mod model;

pub use dimension::{BaseDimension, Dimension};
pub use error::{DimensionError, ModelError};
pub use expression::{Expression, ExpressionArena};
pub use ids::{ExprId, ParameterId, StateId};
pub use model::{DerivativeEquation, Model, ModelBuilder, Parameter, State};
