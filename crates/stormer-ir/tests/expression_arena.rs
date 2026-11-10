use stormer_ir::{ExprId, Expression, ExpressionArena, ModelError};

#[test]
fn arena_assigns_stable_expression_ids() {
    let mut arena = ExpressionArena::new();

    let first = arena.insert(Expression::Constant(1.0));
    let second = arena.insert(Expression::Constant(2.0));

    assert_eq!(first, ExprId::from_index(0));
    assert_eq!(second, ExprId::from_index(1));
    assert_eq!(arena.len(), 2);
}

#[test]
fn arena_preserves_inserted_nodes() {
    let mut arena = ExpressionArena::new();
    let expression = Expression::Constant(2.5);

    let id = arena.insert(expression.clone());

    assert_eq!(arena.get(id), Ok(&expression));
}

#[test]
fn arena_rejects_an_invalid_expression_id() {
    let arena = ExpressionArena::new();
    let missing = ExprId::from_index(7);

    assert_eq!(
        arena.get(missing),
        Err(ModelError::InvalidExpressionId { id: missing })
    );
}

#[test]
fn arena_detects_an_invalid_operand_reference() {
    let mut arena = ExpressionArena::new();
    let missing_operand = ExprId::from_index(99);
    let expression = arena.insert(Expression::Negate {
        operand: missing_operand,
    });

    assert_eq!(
        arena.validate_operands(),
        Err(ModelError::UnavailableExpressionOperand {
            expression,
            operand: missing_operand,
        })
    );
}

#[test]
fn arena_rejects_forward_operand_reference_to_enforce_dag_order() {
    let mut arena = ExpressionArena::new();
    let forward_operand = ExprId::from_index(1);
    let first = arena.insert(Expression::Negate {
        operand: forward_operand,
    });
    let _second = arena.insert(Expression::Constant(1.0));

    assert_eq!(
        arena.validate_operands(),
        Err(ModelError::UnavailableExpressionOperand {
            expression: first,
            operand: forward_operand,
        })
    );
}
