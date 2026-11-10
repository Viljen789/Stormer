#![forbid(unsafe_code)]
//! Source-language parser, unit resolver, semantic checker, and IR lowering for Størmer.

mod checker;
mod diagnostic;
mod parser;
mod units;

pub use checker::{compile_str, lower_ast};
pub use diagnostic::{Diagnostic, DiagnosticKind, Span};
pub use parser::{
    AstDeclaration, AstDerivative, AstExpr, AstModel, AstStatement, BinaryOp, parse_str,
};
pub use stormer_ir::Model;
pub use units::parse_unit_dimension;
