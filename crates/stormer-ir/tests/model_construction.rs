use stormer_ir::{
    BaseDimension, Dimension, DimensionError, ExprId, Expression, ModelBuilder, ModelError,
    ParameterId, StateId,
};

#[test]
fn parameter_and_state_ids_are_distinct() {
    let mut builder = ModelBuilder::new();
    let dimensionless = Dimension::dimensionless();

    let parameter_id: ParameterId = builder.add_parameter("gain".to_owned(), dimensionless, 1.0);
    let state_id: StateId = builder.add_state("position".to_owned(), dimensionless, 0.0);

    assert_eq!(parameter_id.index(), 0);
    assert_eq!(state_id.index(), 0);
}

#[test]
fn builder_preserves_declaration_order() {
    let mut builder = ModelBuilder::new();
    let dimensionless = Dimension::dimensionless();

    builder.add_parameter("first".to_owned(), dimensionless, 1.0);
    builder.add_parameter("second".to_owned(), dimensionless, 2.0);

    let model = builder
        .finish()
        .expect("parameter-only model should finalize");
    let names: Vec<_> = model.parameters().iter().map(|item| item.name()).collect();

    assert_eq!(names, ["first", "second"]);
}

#[test]
fn builder_rejects_an_invalid_derivative_target() {
    let mut builder = ModelBuilder::new();
    let missing_state = StateId::from_index(3);

    assert_eq!(
        builder.assign_derivative(missing_state, ExprId::from_index(0)),
        Err(ModelError::UnavailableDerivativeTarget {
            state: missing_state,
        })
    );
}

#[test]
fn builder_rejects_duplicate_derivatives() {
    let mut builder = ModelBuilder::new();
    let state = builder.add_state("position".to_owned(), Dimension::dimensionless(), 0.0);
    let expression = builder.insert_expression(Expression::Constant(0.0));

    builder
        .assign_derivative(state, expression)
        .expect("the first derivative should be accepted");

    assert_eq!(
        builder.assign_derivative(state, expression),
        Err(ModelError::DuplicateDerivativeEquation { state })
    );
}

#[test]
fn builder_rejects_states_without_derivatives() {
    let mut builder = ModelBuilder::new();
    let state = builder.add_state("position".to_owned(), Dimension::dimensionless(), 1.0);

    assert_eq!(
        builder.finish(),
        Err(ModelError::MissingDerivativeEquation { state })
    );
}

#[test]
fn finalized_model_exposes_consistent_read_only_data() {
    let mut builder = ModelBuilder::new();
    let dimensionless = Dimension::dimensionless();
    let parameter = builder.add_parameter("gain".to_owned(), dimensionless, 2.0);
    let state = builder.add_state("position".to_owned(), dimensionless, 1.0);
    let parameter_expression = builder.insert_expression(Expression::Parameter(parameter));

    builder
        .assign_derivative(state, parameter_expression)
        .expect("a derivative for an available state should be accepted");

    let model = builder
        .finish()
        .expect("the complete model should finalize");

    assert_eq!(model.parameters().len(), 1);
    assert_eq!(model.parameters()[0].name(), "gain");
    assert_eq!(model.states().len(), 1);
    assert_eq!(model.states()[0].name(), "position");
    assert_eq!(model.expressions().len(), 1);
    assert_eq!(model.derivatives().len(), 1);
    assert_eq!(model.derivatives()[0].state(), state);
    assert_eq!(model.derivatives()[0].expression(), parameter_expression);
}

#[test]
fn builder_rejects_an_invalid_parameter_reference_during_finalization() {
    let mut builder = ModelBuilder::new();
    let missing_parameter = ParameterId::from_index(4);
    builder.insert_expression(Expression::Parameter(missing_parameter));

    assert_eq!(
        builder.finish(),
        Err(ModelError::InvalidParameterId {
            id: missing_parameter,
        })
    );
}

#[test]
fn builder_rejects_an_invalid_state_reference_during_finalization() {
    let mut builder = ModelBuilder::new();
    let missing_state = StateId::from_index(4);
    builder.insert_expression(Expression::State(missing_state));

    assert_eq!(
        builder.finish(),
        Err(ModelError::InvalidStateId { id: missing_state })
    );
}

#[test]
fn builder_rejects_an_invalid_derivative_root_during_finalization() {
    let mut builder = ModelBuilder::new();
    let state = builder.add_state("position".to_owned(), Dimension::dimensionless(), 0.0);
    let missing_expression = ExprId::from_index(4);
    builder
        .assign_derivative(state, missing_expression)
        .expect("the available state should accept one derivative assignment");

    assert_eq!(
        builder.finish(),
        Err(ModelError::InvalidExpressionId {
            id: missing_expression,
        })
    );
}

#[test]
fn builder_rejects_an_invalid_operand_during_finalization() {
    let mut builder = ModelBuilder::new();
    let state = builder.add_state("position".to_owned(), Dimension::dimensionless(), 0.0);
    let missing_operand = ExprId::from_index(9);
    let root = builder.insert_expression(Expression::Negate {
        operand: missing_operand,
    });
    builder
        .assign_derivative(state, root)
        .expect("the available state should accept one derivative assignment");

    assert_eq!(
        builder.finish(),
        Err(ModelError::UnavailableExpressionOperand {
            expression: root,
            operand: missing_operand,
        })
    );
}

#[test]
fn dimensional_validation_rejects_incompatible_addition_and_derivative_units() {
    let mut builder = ModelBuilder::new();
    let length = Dimension::base(BaseDimension::Length);
    let mass = Dimension::base(BaseDimension::Mass);

    let p_mass = builder.add_parameter("m".to_owned(), mass, 1.0);
    let s_pos = builder.add_state("x".to_owned(), length, 0.0);

    let e_mass = builder.insert_expression(Expression::Parameter(p_mass));
    let e_pos = builder.insert_expression(Expression::State(s_pos));
    let bad_sum = builder.insert_expression(Expression::Add {
        left: e_mass,
        right: e_pos,
    });
    builder.assign_derivative(s_pos, bad_sum).unwrap();
    let model = builder.finish().unwrap();

    assert_eq!(
        model.validate_dimensions(),
        Err(DimensionError::IncompatibleOperands {
            expression: bad_sum,
            left: mass,
            right: length,
        })
    );
}
