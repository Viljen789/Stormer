use stormer_frontend::{DiagnosticKind, compile_str};
use stormer_ir::{BaseDimension, Dimension};

#[test]
fn compiles_canonical_damped_oscillator_source() {
    let source = "
        # Canonical 1D damped harmonic oscillator
        parameter mass: kg = 1.0
        parameter damping: kg/s = 0.5
        parameter stiffness: N/m = 4.0

        state position: m = 1.0
        state velocity: m/s = 0.0

        derivative position = velocity
        derivative velocity = -(damping / mass) * velocity - (stiffness / mass) * position
    ";

    let model = compile_str(source).expect("valid damped oscillator source should compile");

    assert_eq!(model.parameters().len(), 3);
    assert_eq!(model.states().len(), 2);
    assert_eq!(model.derivatives().len(), 2);

    let m_dim = Dimension::base(BaseDimension::Mass);
    let l_dim = Dimension::base(BaseDimension::Length);
    let t_dim = Dimension::base(BaseDimension::Time);

    assert_eq!(model.parameters()[0].name(), "mass");
    assert_eq!(model.parameters()[0].dimension(), m_dim);
    assert_eq!(model.parameters()[1].name(), "damping");
    assert_eq!(model.parameters()[1].dimension(), m_dim.divide(t_dim));
    assert_eq!(model.parameters()[2].name(), "stiffness");
    assert_eq!(
        model.parameters()[2].dimension(),
        m_dim.divide(t_dim.pow(2))
    );

    assert_eq!(model.states()[0].name(), "position");
    assert_eq!(model.states()[0].dimension(), l_dim);
    assert_eq!(model.states()[1].name(), "velocity");
    assert_eq!(model.states()[1].dimension(), l_dim.divide(t_dim));
}

#[test]
fn reports_incompatible_dimensions_in_addition() {
    let source = "
        parameter mass: kg = 1.0
        state position: m = 1.0
        derivative position = mass + position
    ";

    let errors = compile_str(source).expect_err("adding mass and length must be rejected");
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].kind(), DiagnosticKind::IncompatibleDimensions);
    assert_eq!(errors[0].span().line, 4);
}

#[test]
fn reports_invalid_derivative_dimension() {
    let source = "
        state position: m = 1.0
        derivative position = position
    ";

    let errors = compile_str(source)
        .expect_err("position derivative with dimension L instead of L/T must fail");
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].kind(), DiagnosticKind::InvalidDerivativeDimension);
}

#[test]
fn reports_undefined_symbol_and_missing_derivative() {
    let source = "
        state position: m = 1.0
        state velocity: m/s = 0.0
        derivative position = unknown_var
    ";

    let errors = compile_str(source).expect_err("undefined symbol must fail");
    assert_eq!(errors[0].kind(), DiagnosticKind::UndefinedSymbol);

    let missing_deriv_source = "
        state position: m = 1.0
    ";
    let missing = compile_str(missing_deriv_source).expect_err("missing derivative must fail");
    assert_eq!(missing[0].kind(), DiagnosticKind::MissingDerivative);
}

#[test]
fn reports_duplicate_declarations_and_unknown_units() {
    let dup_source = "
        parameter k: N/m = 1.0
        state k: m = 0.0
        derivative k = k
    ";
    let dup_errors = compile_str(dup_source).expect_err("duplicate declaration must fail");
    assert!(
        dup_errors
            .iter()
            .any(|d| d.kind() == DiagnosticKind::DuplicateDeclaration)
    );

    let bad_unit_source = "
        parameter k: parsecs = 1.0
    ";
    let unit_errors = compile_str(bad_unit_source).expect_err("unknown unit must fail");
    assert_eq!(unit_errors[0].kind(), DiagnosticKind::UnknownUnit);
}
