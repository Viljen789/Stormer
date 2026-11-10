use stormer_ir::{BaseDimension, Dimension, Expression, Model, ModelBuilder, StateId};

fn build_harmonic_oscillator() -> Model {
    let mut builder = ModelBuilder::new();

    let mass_dim = Dimension::base(BaseDimension::Mass);
    let length_dim = Dimension::base(BaseDimension::Length);
    let time_dim = Dimension::base(BaseDimension::Time);
    let velocity_dim = length_dim.divide(time_dim);
    let stiffness_dim = mass_dim
        .multiply(length_dim)
        .divide(time_dim)
        .divide(time_dim)
        .divide(length_dim);

    let mass = builder.add_parameter("mass".to_owned(), mass_dim, 1.0);
    let stiffness = builder.add_parameter("stiffness".to_owned(), stiffness_dim, 4.0);

    let position = builder.add_state("position".to_owned(), length_dim, 1.0);
    let velocity = builder.add_state("velocity".to_owned(), velocity_dim, 0.0);

    let velocity_ref = builder.insert_expression(Expression::State(velocity));
    let stiffness_ref = builder.insert_expression(Expression::Parameter(stiffness));
    let mass_ref = builder.insert_expression(Expression::Parameter(mass));
    let ratio = builder.insert_expression(Expression::Divide {
        left: stiffness_ref,
        right: mass_ref,
    });
    let negated_ratio = builder.insert_expression(Expression::Negate { operand: ratio });
    let position_ref = builder.insert_expression(Expression::State(position));
    let acceleration = builder.insert_expression(Expression::Multiply {
        left: negated_ratio,
        right: position_ref,
    });

    builder
        .assign_derivative(position, velocity_ref)
        .expect("position derivative should be accepted");
    builder
        .assign_derivative(velocity, acceleration)
        .expect("velocity derivative should be accepted");

    let model = builder
        .finish()
        .expect("harmonic oscillator model should be structurally valid");
    model
        .validate_dimensions()
        .expect("harmonic oscillator model should be dimensionally valid");
    model
}

#[test]
fn harmonic_oscillator_can_be_constructed_programmatically() {
    let model = build_harmonic_oscillator();

    assert!(!model.expressions().is_empty());
}

#[test]
fn harmonic_oscillator_uses_two_states_and_two_parameters() {
    let model = build_harmonic_oscillator();

    assert_eq!(model.parameters().len(), 2);
    assert_eq!(model.states().len(), 2);
    assert_eq!(model.derivatives().len(), 2);

    let mass = &model.parameters()[0];
    assert_eq!(mass.name(), "mass");
    assert_eq!(mass.dimension(), Dimension::base(BaseDimension::Mass));
    assert_eq!(mass.initial_value().to_bits(), 1.0_f64.to_bits());

    let time = Dimension::base(BaseDimension::Time);
    let stiffness_dimension = Dimension::base(BaseDimension::Mass)
        .multiply(Dimension::base(BaseDimension::Length))
        .divide(time)
        .divide(time)
        .divide(Dimension::base(BaseDimension::Length));
    let stiffness = &model.parameters()[1];
    assert_eq!(stiffness.name(), "stiffness");
    assert_eq!(stiffness.dimension(), stiffness_dimension);
    assert_eq!(stiffness.initial_value().to_bits(), 4.0_f64.to_bits());

    let position = &model.states()[0];
    assert_eq!(position.name(), "position");
    assert_eq!(position.dimension(), Dimension::base(BaseDimension::Length));
    assert_eq!(position.initial_value().to_bits(), 1.0_f64.to_bits());

    let velocity = &model.states()[1];
    assert_eq!(velocity.name(), "velocity");
    assert_eq!(
        velocity.dimension(),
        Dimension::base(BaseDimension::Length).divide(time)
    );
    assert_eq!(velocity.initial_value().to_bits(), 0.0_f64.to_bits());
}

#[test]
fn harmonic_oscillator_derivative_roots_reference_valid_expressions() {
    let model = build_harmonic_oscillator();
    let position = StateId::from_index(0);
    let velocity = StateId::from_index(1);
    let position_derivative = model
        .derivatives()
        .iter()
        .find(|equation| equation.state() == position)
        .expect("position must have a derivative equation");
    let velocity_derivative = model
        .derivatives()
        .iter()
        .find(|equation| equation.state() == velocity)
        .expect("velocity must have a derivative equation");

    assert_eq!(
        model.expressions().get(position_derivative.expression()),
        Ok(&Expression::State(velocity))
    );

    let Expression::Multiply {
        left: negated_ratio,
        right: position_reference,
    } = model
        .expressions()
        .get(velocity_derivative.expression())
        .expect("velocity derivative root must exist")
    else {
        panic!("velocity derivative root must multiply the negated ratio by position");
    };
    assert_eq!(
        model.expressions().get(*position_reference),
        Ok(&Expression::State(position))
    );

    let Expression::Negate { operand: ratio } = model
        .expressions()
        .get(*negated_ratio)
        .expect("negated stiffness-to-mass ratio must exist")
    else {
        panic!("velocity derivative must negate the stiffness-to-mass ratio");
    };
    let Expression::Divide {
        left: stiffness_reference,
        right: mass_reference,
    } = model
        .expressions()
        .get(*ratio)
        .expect("stiffness-to-mass ratio must exist")
    else {
        panic!("velocity derivative must divide stiffness by mass");
    };

    assert_eq!(
        model.expressions().get(*stiffness_reference),
        Ok(&Expression::Parameter(stormer_ir::ParameterId::from_index(
            1
        )))
    );
    assert_eq!(
        model.expressions().get(*mass_reference),
        Ok(&Expression::Parameter(stormer_ir::ParameterId::from_index(
            0
        )))
    );
}
