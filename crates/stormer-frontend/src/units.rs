use stormer_ir::{BaseDimension, Dimension};

use crate::{Diagnostic, DiagnosticKind, Span};

/// Parses a unit expression (such as `kg`, `m/s`, `N/m`, `kg/s`, `m/s^2`, or `1`) into its
/// physical [`Dimension`].
pub fn parse_unit_dimension(unit_text: &str, span: Span) -> Result<Dimension, Diagnostic> {
    let trimmed = unit_text.trim();
    if trimmed.is_empty() {
        return Err(Diagnostic::new(
            DiagnosticKind::UnknownUnit,
            "missing physical unit annotation",
            span,
        ));
    }

    let mut parser = UnitParser::new(trimmed, span);
    let dim = parser.parse_expr()?;
    parser.skip_whitespace();
    if !parser.is_eof() {
        return Err(Diagnostic::new(
            DiagnosticKind::UnknownUnit,
            format!("unexpected trailing unit syntax in `{trimmed}`"),
            span,
        ));
    }
    Ok(dim)
}

struct UnitParser<'a> {
    input: &'a [u8],
    raw: &'a str,
    pos: usize,
    span: Span,
}

impl<'a> UnitParser<'a> {
    fn new(raw: &'a str, span: Span) -> Self {
        Self {
            input: raw.as_bytes(),
            raw,
            pos: 0,
            span,
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    fn peek(&mut self) -> Option<u8> {
        self.skip_whitespace();
        self.input.get(self.pos).copied()
    }

    fn parse_expr(&mut self) -> Result<Dimension, Diagnostic> {
        let mut acc = self.parse_power()?;
        while let Some(op) = self.peek() {
            if op == b'*' {
                self.pos += 1;
                let rhs = self.parse_power()?;
                acc = acc.multiply(rhs);
            } else if op == b'/' {
                self.pos += 1;
                let rhs = self.parse_power()?;
                acc = acc.divide(rhs);
            } else {
                break;
            }
        }
        Ok(acc)
    }

    fn parse_power(&mut self) -> Result<Dimension, Diagnostic> {
        let base = self.parse_atom()?;
        if self.peek() == Some(b'^') {
            self.pos += 1;
            let exp = self.parse_exponent()?;
            Ok(base.pow(exp))
        } else {
            Ok(base)
        }
    }

    fn parse_exponent(&mut self) -> Result<i8, Diagnostic> {
        self.skip_whitespace();
        let start = self.pos;
        if self.pos < self.input.len()
            && (self.input[self.pos] == b'-' || self.input[self.pos] == b'+')
        {
            self.pos += 1;
        }
        let digits_start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        if self.pos == digits_start {
            return Err(Diagnostic::new(
                DiagnosticKind::UnknownUnit,
                format!("expected integer exponent in unit `{}`", self.raw),
                self.span,
            ));
        }
        self.raw[start..self.pos].parse::<i8>().map_err(|_| {
            Diagnostic::new(
                DiagnosticKind::UnknownUnit,
                format!("invalid unit exponent in `{}`", self.raw),
                self.span,
            )
        })
    }

    fn parse_atom(&mut self) -> Result<Dimension, Diagnostic> {
        self.skip_whitespace();
        if self.peek() == Some(b'(') {
            self.pos += 1;
            let inner = self.parse_expr()?;
            if self.peek() != Some(b')') {
                return Err(Diagnostic::new(
                    DiagnosticKind::UnknownUnit,
                    format!("unclosed parenthesis in unit `{}`", self.raw),
                    self.span,
                ));
            }
            self.pos += 1;
            return Ok(inner);
        }

        if self.peek() == Some(b'1') {
            self.pos += 1;
            return Ok(Dimension::dimensionless());
        }

        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_alphabetic() {
            self.pos += 1;
        }
        if self.pos == start {
            return Err(Diagnostic::new(
                DiagnosticKind::UnknownUnit,
                format!("invalid unit syntax `{}`", self.raw),
                self.span,
            ));
        }

        let symbol = &self.raw[start..self.pos];
        let mass = Dimension::base(BaseDimension::Mass);
        let length = Dimension::base(BaseDimension::Length);
        let time = Dimension::base(BaseDimension::Time);
        let current = Dimension::base(BaseDimension::ElectricCurrent);
        let temperature = Dimension::base(BaseDimension::ThermodynamicTemperature);
        let substance = Dimension::base(BaseDimension::AmountOfSubstance);
        let luminous = Dimension::base(BaseDimension::LuminousIntensity);

        match symbol {
            "kg" | "M" => Ok(mass),
            "m" | "L" => Ok(length),
            "s" | "T" => Ok(time),
            "A" | "I" => Ok(current),
            "K" | "Theta" => Ok(temperature),
            "mol" => Ok(substance),
            "cd" => Ok(luminous),
            "Hz" => Ok(Dimension::dimensionless().divide(time)),
            "N" => Ok(mass.multiply(length).divide(time.pow(2))),
            "J" => Ok(mass.multiply(length.pow(2)).divide(time.pow(2))),
            "W" => Ok(mass.multiply(length.pow(2)).divide(time.pow(3))),
            "Pa" => Ok(mass.divide(length).divide(time.pow(2))),
            _ => Err(Diagnostic::new(
                DiagnosticKind::UnknownUnit,
                format!("unknown physical unit `{symbol}`"),
                self.span,
            )),
        }
    }
}
