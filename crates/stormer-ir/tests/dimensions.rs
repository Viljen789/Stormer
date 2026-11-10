use stormer_ir::{BaseDimension, Dimension};

const BASE_DIMENSIONS: [BaseDimension; 7] = [
    BaseDimension::Mass,
    BaseDimension::Length,
    BaseDimension::Time,
    BaseDimension::ElectricCurrent,
    BaseDimension::ThermodynamicTemperature,
    BaseDimension::AmountOfSubstance,
    BaseDimension::LuminousIntensity,
];

#[test]
fn dimensionless_has_zero_exponents() {
    let dimensionless = Dimension::dimensionless();

    for base in BASE_DIMENSIONS {
        assert_eq!(dimensionless.exponent(base), 0);
    }
}

#[test]
fn base_dimensions_are_independent() {
    for selected in BASE_DIMENSIONS {
        let dimension = Dimension::base(selected);

        for inspected in BASE_DIMENSIONS {
            let expected = i8::from(selected == inspected);
            assert_eq!(dimension.exponent(inspected), expected);
        }
    }
}

#[test]
fn multiplication_composes_dimensions() {
    let mass_length =
        Dimension::base(BaseDimension::Mass).multiply(Dimension::base(BaseDimension::Length));

    assert_eq!(mass_length.exponent(BaseDimension::Mass), 1);
    assert_eq!(mass_length.exponent(BaseDimension::Length), 1);
    assert_eq!(mass_length.exponent(BaseDimension::Time), 0);
}

#[test]
fn division_composes_dimensions() {
    let length_per_time =
        Dimension::base(BaseDimension::Length).divide(Dimension::base(BaseDimension::Time));

    assert_eq!(length_per_time.exponent(BaseDimension::Length), 1);
    assert_eq!(length_per_time.exponent(BaseDimension::Time), -1);
    assert_eq!(length_per_time.exponent(BaseDimension::Mass), 0);
}

#[test]
fn velocity_equals_length_divided_by_time() {
    let velocity =
        Dimension::base(BaseDimension::Length).divide(Dimension::base(BaseDimension::Time));

    assert_eq!(velocity.exponent(BaseDimension::Length), 1);
    assert_eq!(velocity.exponent(BaseDimension::Time), -1);
    assert_eq!(velocity.exponent(BaseDimension::Mass), 0);
}

#[test]
fn force_equals_mass_times_acceleration() {
    let time = Dimension::base(BaseDimension::Time);
    let acceleration = Dimension::base(BaseDimension::Length)
        .divide(time)
        .divide(time);
    let force = Dimension::base(BaseDimension::Mass).multiply(acceleration);

    assert_eq!(force.exponent(BaseDimension::Mass), 1);
    assert_eq!(force.exponent(BaseDimension::Length), 1);
    assert_eq!(force.exponent(BaseDimension::Time), -2);
}

#[test]
fn equivalent_dimensions_are_compatible() {
    let time = Dimension::base(BaseDimension::Time);
    let direct_velocity = Dimension::base(BaseDimension::Length).divide(time);
    let composed_velocity = Dimension::dimensionless()
        .multiply(Dimension::base(BaseDimension::Length))
        .divide(time);

    assert!(direct_velocity.is_compatible_with(composed_velocity));
}

#[test]
fn different_dimensions_are_incompatible() {
    let mass = Dimension::base(BaseDimension::Mass);
    let length = Dimension::base(BaseDimension::Length);

    assert!(!mass.is_compatible_with(length));
}

#[test]
fn dimension_formatting_is_canonical() {
    let time = Dimension::base(BaseDimension::Time);
    let velocity = Dimension::base(BaseDimension::Length).divide(time);
    let force = Dimension::base(BaseDimension::Mass).multiply(velocity.divide(time));

    assert_eq!(Dimension::dimensionless().to_string(), "1");
    assert_eq!(velocity.to_string(), "L T^-1");
    assert_eq!(force.to_string(), "M L T^-2");
}
