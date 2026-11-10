use crate::{Diagnostic, DiagnosticKind, Span};

/// A parsed declaration or derivative statement in a `.storm` source file.
#[derive(Debug, Clone, PartialEq)]
pub enum AstStatement {
    /// `parameter <name>: <unit> = <value>`
    Parameter(AstDeclaration),
    /// `state <name>: <unit> = <value>`
    State(AstDeclaration),
    /// `derivative <state_name> = <expr>`
    Derivative(AstDerivative),
}

/// A parsed parameter or state declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct AstDeclaration {
    /// Identifier of the declared quantity.
    pub name: String,
    /// Span of the identifier.
    pub name_span: Span,
    /// Raw unit annotation string.
    pub unit_text: String,
    /// Span of the unit annotation.
    pub unit_span: Span,
    /// Initial numeric value.
    pub initial_value: f64,
    /// Span of the initial value.
    pub value_span: Span,
}

/// A parsed derivative equation associating a target state name with an expression AST.
#[derive(Debug, Clone, PartialEq)]
pub struct AstDerivative {
    /// Target state identifier.
    pub target: String,
    /// Span of the target state identifier.
    pub target_span: Span,
    /// Root of the parsed derivative expression.
    pub expression: AstExpr,
}

/// Binary arithmetic operators in the source language.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    /// Addition (`+`).
    Add,
    /// Subtraction (`-`).
    Subtract,
    /// Multiplication (`*`).
    Multiply,
    /// Division (`/`).
    Divide,
}

/// Parsed expression syntax tree node with source span.
#[derive(Debug, Clone, PartialEq)]
pub enum AstExpr {
    /// Dimensionless numeric literal.
    Number {
        /// Parsed numeric value.
        value: f64,
        /// Span of the literal.
        span: Span,
    },
    /// Reference to a declared parameter or state name.
    Identifier {
        /// Referenced identifier name.
        name: String,
        /// Span of the identifier.
        span: Span,
    },
    /// Unary negation (`-expr`).
    Negate {
        /// Negated operand.
        operand: Box<AstExpr>,
        /// Enclosing span of the unary expression.
        span: Span,
    },
    /// Binary arithmetic operation.
    Binary {
        /// Operator kind.
        op: BinaryOp,
        /// Left operand.
        left: Box<AstExpr>,
        /// Right operand.
        right: Box<AstExpr>,
        /// Enclosing span of the binary expression.
        span: Span,
    },
}

impl AstExpr {
    /// Returns the source span of this expression node.
    #[must_use]
    pub fn span(&self) -> Span {
        match self {
            Self::Number { span, .. }
            | Self::Identifier { span, .. }
            | Self::Negate { span, .. }
            | Self::Binary { span, .. } => *span,
        }
    }
}

/// A complete parsed `.storm` syntax tree before semantic resolution.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AstModel {
    /// Statements in source order.
    pub statements: Vec<AstStatement>,
}

/// Parses `.storm` source text into an [`AstModel`].
pub fn parse_str(source: &str) -> Result<AstModel, Vec<Diagnostic>> {
    let mut statements = Vec::new();
    let mut diagnostics = Vec::new();
    let mut byte_offset = 0;

    for (line_idx, raw_line) in source.split('\n').enumerate() {
        let line_number = line_idx + 1;
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        let uncommented = strip_comment(line);
        let trimmed = uncommented.trim();

        if !trimmed.is_empty() {
            match parse_statement_line(uncommented, byte_offset, line_number) {
                Ok(stmt) => statements.push(stmt),
                Err(diag) => diagnostics.push(diag),
            }
        }

        byte_offset += raw_line.len() + 1;
    }

    if diagnostics.is_empty() {
        Ok(AstModel { statements })
    } else {
        Err(diagnostics)
    }
}

fn strip_comment(line: &str) -> &str {
    let hash_pos = line.find('#');
    let slash_pos = line.find("//");
    match (hash_pos, slash_pos) {
        (Some(a), Some(b)) => &line[..a.min(b)],
        (Some(a), None) => &line[..a],
        (None, Some(b)) => &line[..b],
        (None, None) => line,
    }
}

fn make_span(
    line_byte_offset: usize,
    line_number: usize,
    start_col0: usize,
    end_col0: usize,
) -> Span {
    Span::new(
        line_byte_offset + start_col0,
        line_byte_offset + end_col0,
        line_number,
        start_col0 + 1,
    )
}

fn parse_statement_line(
    line: &str,
    line_byte_offset: usize,
    line_number: usize,
) -> Result<AstStatement, Diagnostic> {
    let first_non_ws = line.find(|c: char| !c.is_whitespace()).unwrap_or(0);
    let rest = &line[first_non_ws..];
    let kw_len = rest.find(|c: char| c.is_whitespace()).unwrap_or(rest.len());
    let keyword = &rest[..kw_len];
    let kw_span = make_span(
        line_byte_offset,
        line_number,
        first_non_ws,
        first_non_ws + kw_len,
    );

    match keyword {
        "parameter" | "param" => {
            let decl =
                parse_declaration(line, first_non_ws + kw_len, line_byte_offset, line_number)?;
            Ok(AstStatement::Parameter(decl))
        }
        "state" => {
            let decl =
                parse_declaration(line, first_non_ws + kw_len, line_byte_offset, line_number)?;
            Ok(AstStatement::State(decl))
        }
        "derivative" => {
            let deriv =
                parse_derivative(line, first_non_ws + kw_len, line_byte_offset, line_number)?;
            Ok(AstStatement::Derivative(deriv))
        }
        _ => Err(Diagnostic::new(
            DiagnosticKind::Syntax,
            format!(
                "expected `parameter`, `state`, or `derivative` declaration, found `{keyword}`"
            ),
            kw_span,
        )),
    }
}

fn parse_declaration(
    line: &str,
    after_kw: usize,
    line_byte_offset: usize,
    line_number: usize,
) -> Result<AstDeclaration, Diagnostic> {
    let tail = &line[after_kw..];
    let colon_rel = tail.find(':').ok_or_else(|| {
        Diagnostic::new(
            DiagnosticKind::Syntax,
            "expected `: <unit> = <value>` in declaration",
            make_span(line_byte_offset, line_number, after_kw, line.len()),
        )
    })?;
    let eq_rel = tail.find('=').ok_or_else(|| {
        Diagnostic::new(
            DiagnosticKind::Syntax,
            "expected `= <value>` in declaration",
            make_span(line_byte_offset, line_number, after_kw, line.len()),
        )
    })?;
    if eq_rel < colon_rel {
        return Err(Diagnostic::new(
            DiagnosticKind::Syntax,
            "unit annotation `: <unit>` must appear before `= <value>`",
            make_span(
                line_byte_offset,
                line_number,
                after_kw + eq_rel,
                after_kw + colon_rel + 1,
            ),
        ));
    }

    let name_slice = &tail[..colon_rel];
    let (name, name_span) =
        extract_trimmed_span(name_slice, after_kw, line_byte_offset, line_number).ok_or_else(
            || {
                Diagnostic::new(
                    DiagnosticKind::Syntax,
                    "missing declaration identifier before `:`",
                    make_span(
                        line_byte_offset,
                        line_number,
                        after_kw,
                        after_kw + colon_rel + 1,
                    ),
                )
            },
        )?;

    if !is_valid_identifier(name) {
        return Err(Diagnostic::new(
            DiagnosticKind::Syntax,
            format!("invalid identifier `{name}`"),
            name_span,
        ));
    }

    let unit_slice = &tail[colon_rel + 1..eq_rel];
    let (unit_text, unit_span) = extract_trimmed_span(
        unit_slice,
        after_kw + colon_rel + 1,
        line_byte_offset,
        line_number,
    )
    .ok_or_else(|| {
        Diagnostic::new(
            DiagnosticKind::UnknownUnit,
            "missing unit annotation between `:` and `=`",
            make_span(
                line_byte_offset,
                line_number,
                after_kw + colon_rel,
                after_kw + eq_rel + 1,
            ),
        )
    })?;

    let value_slice = &tail[eq_rel + 1..];
    let (value_text, value_span) = extract_trimmed_span(
        value_slice,
        after_kw + eq_rel + 1,
        line_byte_offset,
        line_number,
    )
    .ok_or_else(|| {
        Diagnostic::new(
            DiagnosticKind::Syntax,
            "missing initial numeric value after `=`",
            make_span(line_byte_offset, line_number, after_kw + eq_rel, line.len()),
        )
    })?;

    let initial_value = value_text.parse::<f64>().map_err(|_| {
        Diagnostic::new(
            DiagnosticKind::Syntax,
            format!("expected finite numeric literal after `=`, found `{value_text}`"),
            value_span,
        )
    })?;
    if !initial_value.is_finite() {
        return Err(Diagnostic::new(
            DiagnosticKind::Syntax,
            format!("initial value `{value_text}` must be finite"),
            value_span,
        ));
    }

    Ok(AstDeclaration {
        name: name.to_owned(),
        name_span,
        unit_text: unit_text.to_owned(),
        unit_span,
        initial_value,
        value_span,
    })
}

fn parse_derivative(
    line: &str,
    after_kw: usize,
    line_byte_offset: usize,
    line_number: usize,
) -> Result<AstDerivative, Diagnostic> {
    let tail = &line[after_kw..];
    let eq_rel = tail.find('=').ok_or_else(|| {
        Diagnostic::new(
            DiagnosticKind::Syntax,
            "expected `= <expression>` in derivative equation",
            make_span(line_byte_offset, line_number, after_kw, line.len()),
        )
    })?;

    let target_slice = &tail[..eq_rel];
    let (target, target_span) =
        extract_trimmed_span(target_slice, after_kw, line_byte_offset, line_number).ok_or_else(
            || {
                Diagnostic::new(
                    DiagnosticKind::Syntax,
                    "missing target state name before `=` in derivative equation",
                    make_span(
                        line_byte_offset,
                        line_number,
                        after_kw,
                        after_kw + eq_rel + 1,
                    ),
                )
            },
        )?;

    if !is_valid_identifier(target) {
        return Err(Diagnostic::new(
            DiagnosticKind::Syntax,
            format!("invalid target state identifier `{target}`"),
            target_span,
        ));
    }

    let expr_offset = after_kw + eq_rel + 1;
    let expr_slice = &line[expr_offset..];
    let mut expr_parser = ExprParser::new(expr_slice, expr_offset, line_byte_offset, line_number);
    let expression = expr_parser.parse_expr()?;
    expr_parser.skip_whitespace();
    if !expr_parser.is_eof() {
        let rem_span = expr_parser.current_span(expr_parser.input.len() - expr_parser.pos);
        return Err(Diagnostic::new(
            DiagnosticKind::Syntax,
            format!(
                "unexpected trailing tokens `{}` in derivative expression",
                &expr_slice[expr_parser.pos..]
            ),
            rem_span,
        ));
    }

    Ok(AstDerivative {
        target: target.to_owned(),
        target_span,
        expression,
    })
}

fn extract_trimmed_span(
    slice: &str,
    base_col0: usize,
    line_byte_offset: usize,
    line_number: usize,
) -> Option<(&str, Span)> {
    let start_rel = slice.find(|c: char| !c.is_whitespace())?;
    let end_rel = slice.rfind(|c: char| !c.is_whitespace()).map(|i| i + 1)?;
    let trimmed = &slice[start_rel..end_rel];
    let span = make_span(
        line_byte_offset,
        line_number,
        base_col0 + start_rel,
        base_col0 + end_rel,
    );
    Some((trimmed, span))
}

fn is_valid_identifier(text: &str) -> bool {
    let mut chars = text.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

struct ExprParser<'a> {
    raw: &'a str,
    input: &'a [u8],
    pos: usize,
    base_col0: usize,
    line_byte_offset: usize,
    line_number: usize,
}

impl<'a> ExprParser<'a> {
    fn new(raw: &'a str, base_col0: usize, line_byte_offset: usize, line_number: usize) -> Self {
        Self {
            raw,
            input: raw.as_bytes(),
            pos: 0,
            base_col0,
            line_byte_offset,
            line_number,
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

    fn current_span(&self, len: usize) -> Span {
        let start_col0 = self.base_col0 + self.pos;
        let end_col0 = start_col0 + len.max(1);
        make_span(
            self.line_byte_offset,
            self.line_number,
            start_col0,
            end_col0,
        )
    }

    fn parse_expr(&mut self) -> Result<AstExpr, Diagnostic> {
        let mut left = self.parse_term()?;
        while let Some(op_byte) = self.peek() {
            let op = match op_byte {
                b'+' => BinaryOp::Add,
                b'-' => BinaryOp::Subtract,
                _ => break,
            };
            self.pos += 1;
            let right = self.parse_term()?;
            let span = left.span().merge(right.span());
            left = AstExpr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<AstExpr, Diagnostic> {
        let mut left = self.parse_unary()?;
        while let Some(op_byte) = self.peek() {
            let op = match op_byte {
                b'*' => BinaryOp::Multiply,
                b'/' => BinaryOp::Divide,
                _ => break,
            };
            self.pos += 1;
            let right = self.parse_unary()?;
            let span = left.span().merge(right.span());
            left = AstExpr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<AstExpr, Diagnostic> {
        self.skip_whitespace();
        if self.peek() == Some(b'-') {
            let minus_span = self.current_span(1);
            self.pos += 1;
            let operand = self.parse_unary()?;
            let span = minus_span.merge(operand.span());
            return Ok(AstExpr::Negate {
                operand: Box::new(operand),
                span,
            });
        }
        if self.peek() == Some(b'+') {
            self.pos += 1;
            return self.parse_unary();
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<AstExpr, Diagnostic> {
        self.skip_whitespace();
        let Some(byte) = self.peek() else {
            return Err(Diagnostic::new(
                DiagnosticKind::Syntax,
                "unexpected end of expression",
                self.current_span(1),
            ));
        };

        if byte == b'(' {
            let open_span = self.current_span(1);
            self.pos += 1;
            let inner = self.parse_expr()?;
            if self.peek() != Some(b')') {
                return Err(Diagnostic::new(
                    DiagnosticKind::Syntax,
                    "unclosed `(` in expression",
                    open_span,
                ));
            }
            let close_span = self.current_span(1);
            self.pos += 1;
            let _ = open_span.merge(close_span);
            return Ok(inner);
        }

        if byte.is_ascii_digit() || byte == b'.' {
            let start = self.pos;
            while self.pos < self.input.len()
                && (self.input[self.pos].is_ascii_digit() || self.input[self.pos] == b'.')
            {
                self.pos += 1;
            }
            if self.pos < self.input.len()
                && (self.input[self.pos] == b'e' || self.input[self.pos] == b'E')
            {
                self.pos += 1;
                if self.pos < self.input.len()
                    && (self.input[self.pos] == b'+' || self.input[self.pos] == b'-')
                {
                    self.pos += 1;
                }
                while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
                    self.pos += 1;
                }
            }
            let span = make_span(
                self.line_byte_offset,
                self.line_number,
                self.base_col0 + start,
                self.base_col0 + self.pos,
            );
            let literal = &self.raw[start..self.pos];
            let value = literal.parse::<f64>().map_err(|_| {
                Diagnostic::new(
                    DiagnosticKind::Syntax,
                    format!("invalid numeric literal `{literal}`"),
                    span,
                )
            })?;
            return Ok(AstExpr::Number { value, span });
        }

        if byte.is_ascii_alphabetic() || byte == b'_' {
            let start = self.pos;
            while self.pos < self.input.len()
                && (self.input[self.pos].is_ascii_alphanumeric() || self.input[self.pos] == b'_')
            {
                self.pos += 1;
            }
            let span = make_span(
                self.line_byte_offset,
                self.line_number,
                self.base_col0 + start,
                self.base_col0 + self.pos,
            );
            let name = self.raw[start..self.pos].to_owned();
            return Ok(AstExpr::Identifier { name, span });
        }

        Err(Diagnostic::new(
            DiagnosticKind::Syntax,
            format!("unexpected character `{}` in expression", byte as char),
            self.current_span(1),
        ))
    }
}
