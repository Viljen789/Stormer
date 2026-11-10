use std::fmt;

/// One of the seven base dimensions in the International System of Quantities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BaseDimension {
    /// Mass, commonly written as M in dimensional formulas.
    Mass,
    /// Length, commonly written as L in dimensional formulas.
    Length,
    /// Time, commonly written as T in dimensional formulas.
    Time,
    /// Electric current, commonly written as I in dimensional formulas.
    ElectricCurrent,
    /// Thermodynamic temperature, commonly written as Theta in dimensional formulas.
    ThermodynamicTemperature,
    /// Amount of substance, commonly written as N in dimensional formulas.
    AmountOfSubstance,
    /// Luminous intensity, commonly written as J in dimensional formulas.
    LuminousIntensity,
}

impl BaseDimension {
    const ALL: [Self; 7] = [
        Self::Mass,
        Self::Length,
        Self::Time,
        Self::ElectricCurrent,
        Self::ThermodynamicTemperature,
        Self::AmountOfSubstance,
        Self::LuminousIntensity,
    ];

    const fn slot(self) -> usize {
        match self {
            Self::Mass => 0,
            Self::Length => 1,
            Self::Time => 2,
            Self::ElectricCurrent => 3,
            Self::ThermodynamicTemperature => 4,
            Self::AmountOfSubstance => 5,
            Self::LuminousIntensity => 6,
        }
    }

    const fn symbol(self) -> &'static str {
        match self {
            Self::Mass => "M",
            Self::Length => "L",
            Self::Time => "T",
            Self::ElectricCurrent => "I",
            Self::ThermodynamicTemperature => "Theta",
            Self::AmountOfSubstance => "N",
            Self::LuminousIntensity => "J",
        }
    }
}

/// A physical dimension represented by one signed exponent for each SI base dimension.
///
/// This value describes dimensions such as mass, velocity, or force.
/// It does not describe named units, conversion scales, or prefixes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dimension {
    exponents: [i8; 7],
}

impl Dimension {
    /// Creates the dimensionless physical dimension.
    #[must_use]
    pub const fn dimensionless() -> Self {
        Self { exponents: [0; 7] }
    }

    /// Creates the physical dimension for one SI base category.
    #[must_use]
    pub const fn base(base: BaseDimension) -> Self {
        let mut exponents = [0; 7];
        exponents[base.slot()] = 1;
        Self { exponents }
    }

    /// Returns the exponent for one SI base category.
    #[must_use]
    pub const fn exponent(&self, base: BaseDimension) -> i8 {
        self.exponents[base.slot()]
    }

    /// Composes two physical dimensions under multiplication.
    #[must_use]
    pub const fn multiply(self, rhs: Self) -> Self {
        let mut exponents = [0; 7];
        let mut i = 0;
        while i < 7 {
            exponents[i] = self.exponents[i] + rhs.exponents[i];
            i += 1;
        }
        Self { exponents }
    }

    /// Composes two physical dimensions under division.
    #[must_use]
    pub const fn divide(self, rhs: Self) -> Self {
        let mut exponents = [0; 7];
        let mut i = 0;
        while i < 7 {
            exponents[i] = self.exponents[i] - rhs.exponents[i];
            i += 1;
        }
        Self { exponents }
    }

    /// Raises a physical dimension to an integer power.
    #[must_use]
    pub const fn pow(self, power: i8) -> Self {
        let mut exponents = [0; 7];
        let mut i = 0;
        while i < 7 {
            exponents[i] = self.exponents[i] * power;
            i += 1;
        }
        Self { exponents }
    }

    /// Reports whether two physical quantities have compatible dimensions.
    #[must_use]
    pub fn is_compatible_with(self, other: Self) -> bool {
        self.exponents == other.exponents
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut wrote_any = false;
        for base in BaseDimension::ALL {
            let exp = self.exponent(base);
            if exp == 0 {
                continue;
            }
            if wrote_any {
                formatter.write_str(" ")?;
            }
            wrote_any = true;
            if exp == 1 {
                formatter.write_str(base.symbol())?;
            } else {
                write!(formatter, "{}^{}", base.symbol(), exp)?;
            }
        }
        if !wrote_any {
            formatter.write_str("1")?;
        }
        Ok(())
    }
}
