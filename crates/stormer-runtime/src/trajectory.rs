/// One complete runtime state observed at one simulation time.
#[derive(Debug, Clone, PartialEq)]
pub struct TrajectorySample {
    pub(crate) time: f64,
    pub(crate) state: Vec<f64>,
}

impl TrajectorySample {
    /// Returns the simulation time associated with this state snapshot.
    #[must_use]
    pub fn time(&self) -> f64 {
        self.time
    }

    /// Returns the complete state snapshot in the system's stable slot order.
    #[must_use]
    pub fn state(&self) -> &[f64] {
        &self.state
    }
}

/// An ordered sequence of complete simulation samples.
///
/// A valid trajectory begins with the requested initial condition.
/// Every sample contains exactly the system's declared number of state entries.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Trajectory {
    pub(crate) samples: Vec<TrajectorySample>,
}

impl Trajectory {
    /// Returns all samples in increasing simulation-time order.
    #[must_use]
    pub fn samples(&self) -> &[TrajectorySample] {
        &self.samples
    }

    /// Returns the number of recorded samples.
    #[must_use]
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Reports whether this trajectory contains no samples.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
}
