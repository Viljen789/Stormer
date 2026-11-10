/// Stable typed index for a parameter declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParameterId(usize);

impl ParameterId {
    /// Wraps a raw index without proving that a matching parameter exists.
    #[must_use]
    pub const fn from_index(index: usize) -> Self {
        Self(index)
    }

    /// Exposes the stored index for inspection and diagnostics.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

/// Stable typed index for a state declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StateId(usize);

impl StateId {
    /// Wraps a raw index without proving that a matching state exists.
    #[must_use]
    pub const fn from_index(index: usize) -> Self {
        Self(index)
    }

    /// Exposes the stored index for inspection and diagnostics.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

/// Stable typed index for an expression node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExprId(usize);

impl ExprId {
    /// Wraps a raw index without proving that a matching expression exists.
    #[must_use]
    pub const fn from_index(index: usize) -> Self {
        Self(index)
    }

    /// Exposes the stored index for inspection and diagnostics.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}
