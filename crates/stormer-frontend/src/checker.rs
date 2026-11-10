use std::collections::HashMap;

use stormer_ir::{
    BaseDimension, Dimension, ExprId, Expression, Model, ModelBuilder, ParameterId, StateId,
};

use crate::{
    AstExpr, AstModel, AstStatement, BinaryOp, Diagnostic, DiagnosticKind, Span, parse_str,
    parse_unit_dimension,
};

#[derive(Debug, Clone, Copy)]
enum SymbolEntry {
    Parameter {
        id: ParameterId,
        dimension: Dimension,
    },
    State {
        id: StateId,
        dimension: Dimension,
    },
}

/// Parses, resolves, dimension-checks, and lowers `.storm` source text into a validated [`Model`].
pub fn compile_str(source: &str) -> Result<Model, Vec<Diagnostic>> {
    let ast = parse_str(source)?;
    lower_ast(&ast)
}

/// Performs name resolution, unit and dimensional analysis, and IR lowering on an [`AstModel`].
pub fn lower_ast(ast: &AstModel) -> Result<Model, Vec<Diagnostic>> {
    let mut builder = ModelBuilder::new();
    let mut symbols: HashMap<String, SymbolEntry> = HashMap::new();
    let mut state_spans: Vec<(StateId, String, Span)> = Vec::new();
    let mut assigned_derivatives: HashMap<StateId, Span> = HashMap::new();
    let mut diagnostics = Vec::new();

    for statement in &ast.statements {
        match statement {
            AstStatement::Parameter(decl) => {
                if symbols.contains_key(&decl.name) {
                    diagnostics.push(Diagnostic::new(
                        DiagnosticKind::DuplicateDeclaration,
                        format!("duplicate declaration of `{}`", decl.name),
                        decl.name_span,
                    ));
                    continue;
                }
                match parse_unit_dimension(&decl.unit_text, decl.unit_span) {
                    Ok(dimension) => {
                        let id =
                            builder.add_parameter(decl.name.clone(), dimension, decl.initial_value);
                        symbols.insert(decl.name.clone(), SymbolEntry::Parameter { id, dimension });
                    }
                    Err(diag) => diagnostics.push(diag),
                }
            }
            AstStatement::State(decl) => {
                if symbols.contains_key(&decl.name) {
                    diagnostics.push(Diagnostic::new(
                        DiagnosticKind::DuplicateDeclaration,
                        format!("duplicate declaration of `{}`", decl.name),
                        decl.name_span,
                    ));
                    continue;
                }
                match parse_unit_dimension(&decl.unit_text, decl.unit_span) {
                    Ok(dimension) => {
                        let id =
                            builder.add_state(decl.name.clone(), dimension, decl.initial_value);
                        symbols.insert(decl.name.clone(), SymbolEntry::State { id, dimension });
                        state_spans.push((id, decl.name.clone(), decl.name_span));
                    }
                    Err(diag) => diagnostics.push(diag),
                }
            }
            AstStatement::Derivative(_) => {}
        }
    }

    let time_dim = Dimension::base(BaseDimension::Time);

    for statement in &ast.statements {
        let AstStatement::Derivative(deriv) = statement else {
            continue;
        };

        let Some(&symbol) = symbols.get(&deriv.target) else {
            diagnostics.push(Diagnostic::new(
                DiagnosticKind::UndefinedSymbol,
                format!("unknown state `{}` in derivative equation", deriv.target),
                deriv.target_span,
            ));
            continue;
        };

        let SymbolEntry::State {
            id: state_id,
            dimension: state_dim,
            ..
        } = symbol
        else {
            diagnostics.push(Diagnostic::new(
                DiagnosticKind::UndefinedSymbol,
                format!(
                    "`{}` is a parameter, not a state variable; only states have derivatives",
                    deriv.target
                ),
                deriv.target_span,
            ));
            continue;
        };

        if assigned_derivatives.contains_key(&state_id) {
            diagnostics.push(Diagnostic::new(
                DiagnosticKind::DuplicateDerivative,
                format!("duplicate derivative equation for state `{}`", deriv.target),
                deriv.target_span,
            ));
            continue;
        }

        match lower_expression(&deriv.expression, &symbols, &mut builder) {
            Ok((expr_id, actual_dim)) => {
                let expected_dim = state_dim.divide(time_dim);
                if !actual_dim.is_compatible_with(expected_dim) {
                    diagnostics.push(Diagnostic::new(
                        DiagnosticKind::InvalidDerivativeDimension,
                        format!(
                            "derivative for state `{}` has dimension `{}`, expected `{}` (state dimension `{}` / T)",
                            deriv.target, actual_dim, expected_dim, state_dim
                        ),
                        deriv.expression.span(),
                    ));
                } else if builder.assign_derivative(state_id, expr_id).is_ok() {
                    assigned_derivatives.insert(state_id, deriv.target_span);
                }
            }
            Err(mut expr_diags) => diagnostics.append(&mut expr_diags),
        }
    }

    if diagnostics.is_empty() {
        for (state_id, name, span) in &state_spans {
            if !assigned_derivatives.contains_key(state_id) {
                diagnostics.push(Diagnostic::new(
                    DiagnosticKind::MissingDerivative,
                    format!("state `{name}` is missing a derivative equation"),
                    *span,
                ));
            }
        }
    }

    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }

    let model = builder.finish().map_err(|err| {
        vec![Diagnostic::new(
            DiagnosticKind::Syntax,
            format!("internal IR finalization error: {err:?}"),
            Span::new(0, 0, 1, 1),
        )]
    })?;

    model.validate_dimensions().map_err(|err| {
        vec![Diagnostic::new(
            DiagnosticKind::InvalidDerivativeDimension,
            format!("IR dimensional validation failed: {err:?}"),
            Span::new(0, 0, 1, 1),
        )]
    })?;

    Ok(model)
}

fn lower_expression(
    expr: &AstExpr,
    symbols: &HashMap<String, SymbolEntry>,
    builder: &mut ModelBuilder,
) -> Result<(ExprId, Dimension), Vec<Diagnostic>> {
    match expr {
        AstExpr::Number { value, .. } => {
            let id = builder.insert_expression(Expression::Constant(*value));
            Ok((id, Dimension::dimensionless()))
        }
        AstExpr::Identifier { name, span } => match symbols.get(name) {
            Some(SymbolEntry::Parameter { id, dimension }) => {
                let expr_id = builder.insert_expression(Expression::Parameter(*id));
                Ok((expr_id, *dimension))
            }
            Some(SymbolEntry::State { id, dimension, .. }) => {
                let expr_id = builder.insert_expression(Expression::State(*id));
                Ok((expr_id, *dimension))
            }
            None => Err(vec![Diagnostic::new(
                DiagnosticKind::UndefinedSymbol,
                format!("undeclared identifier `{name}`"),
                *span,
            )]),
        },
        AstExpr::Negate { operand, .. } => {
            let (inner_id, dim) = lower_expression(operand, symbols, builder)?;
            let id = builder.insert_expression(Expression::Negate { operand: inner_id });
            Ok((id, dim))
        }
        AstExpr::Binary {
            op,
            left,
            right,
            span,
        } => {
            let left_res = lower_expression(left, symbols, builder);
            let right_res = lower_expression(right, symbols, builder);
            let ((left_id, left_dim), (right_id, right_dim)) = match (left_res, right_res) {
                (Ok(l), Ok(r)) => (l, r),
                (Err(mut d1), Err(mut d2)) => {
                    d1.append(&mut d2);
                    return Err(d1);
                }
                (Err(d), Ok(_)) | (Ok(_), Err(d)) => return Err(d),
            };

            match op {
                BinaryOp::Add | BinaryOp::Subtract => {
                    if !left_dim.is_compatible_with(right_dim) {
                        return Err(vec![Diagnostic::new(
                            DiagnosticKind::IncompatibleDimensions,
                            format!(
                                "cannot combine incompatible dimensions `{left_dim}` and `{right_dim}`"
                            ),
                            *span,
                        )]);
                    }
                    let node = if *op == BinaryOp::Add {
                        Expression::Add {
                            left: left_id,
                            right: right_id,
                        }
                    } else {
                        Expression::Subtract {
                            left: left_id,
                            right: right_id,
                        }
                    };
                    let id = builder.insert_expression(node);
                    Ok((id, left_dim))
                }
                BinaryOp::Multiply => {
                    let id = builder.insert_expression(Expression::Multiply {
                        left: left_id,
                        right: right_id,
                    });
                    Ok((id, left_dim.multiply(right_dim)))
                }
                BinaryOp::Divide => {
                    let id = builder.insert_expression(Expression::Divide {
                        left: left_id,
                        right: right_id,
                    });
                    Ok((id, left_dim.divide(right_dim)))
                }
            }
        }
    }
}
