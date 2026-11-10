use std::fmt;

/// Source byte range and 1-based line/column position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// Byte offset of the start of the token or construct.
    pub start: usize,
    /// Byte offset of the end of the token or construct.
    pub end: usize,
    /// 1-based line number.
    pub line: usize,
    /// 1-based column number.
    pub column: usize,
}

impl Span {
    /// Creates a source span.
    #[must_use]
    pub const fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }

    /// Combines two spans on the same source into an enclosing span.
    #[must_use]
    pub const fn merge(self, other: Self) -> Self {
        let start = if self.start < other.start {
            self.start
        } else {
            other.start
        };
        let end = if self.end > other.end {
            self.end
        } else {
            other.end
        };
        Self {
            start,
            end,
            line: self.line,
            column: self.column,
        }
    }
}

/// Classification of a frontend syntax or semantic diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticKind {
    /// Malformed source syntax.
    Syntax,
    /// Unrecognized or malformed physical unit notation.
    UnknownUnit,
    /// Duplicate parameter or state declaration name.
    DuplicateDeclaration,
    /// Reference to an undeclared identifier or non-state derivative target.
    UndefinedSymbol,
    /// Addition or subtraction of expressions with incompatible physical dimensions.
    IncompatibleDimensions,
    /// Derivative expression dimension does not match `state_dimension / Time`.
    InvalidDerivativeDimension,
    /// More than one derivative equation declared for the same state.
    DuplicateDerivative,
    /// Declared state has no derivative equation.
    MissingDerivative,
}

/// A structured source-level diagnostic with span provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    kind: DiagnosticKind,
    message: String,
    span: Span,
}

impl Diagnostic {
    /// Creates a new frontend diagnostic.
    #[must_use]
    pub fn new(kind: DiagnosticKind, message: impl Into<String>, span: Span) -> Self {
        Self {
            kind,
            message: message.into(),
            span,
        }
    }

    /// Returns the diagnostic category.
    #[must_use]
    pub fn kind(&self) -> DiagnosticKind {
        self.kind
    }

    /// Returns the human-readable diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the source span associated with this diagnostic.
    #[must_use]
    pub fn span(&self) -> Span {
        self.span
    }

    /// Formats the diagnostic with a source snippet and underline marker.
    #[must_use]
    pub fn render(&self, source: &str) -> String {
        let mut out = format!(
            "error[{:?}]: {}\n  --> line {}:{}\n",
            self.kind, self.message, self.span.line, self.span.column
        );
        if let Some(line_text) = source.lines().nth(self.span.line.saturating_sub(1)) {
            let caret_offset = self.span.column.saturating_sub(1);
            let width = self.span.end.saturating_sub(self.span.start).max(1);
            out.push_str(&format!("   |\n{:>2} | {}\n", self.span.line, line_text));
            out.push_str(&format!(
                "   | {}{}\n",
                " ".repeat(caret_offset),
                "^".repeat(width.min(line_text.len().saturating_sub(caret_offset).max(1)))
            ));
        }
        out
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}: {:?}: {}",
            self.span.line, self.span.column, self.kind, self.message
        )
    }
}
