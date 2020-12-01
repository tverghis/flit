/// Possible errors returned when attempting to create `BloomFilter` with faulty arguments.
#[derive(Debug)]
pub enum CreationError {
    /// False positive rate must be between 0 and 1 (non-inclusive).
    InvalidFalsePositiveRange(f64),
    /// Number of estimated items must be greater than zero.
    InvalidEstimatedItems(usize),
}

impl std::fmt::Display for CreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self)
    }
}

impl std::error::Error for CreationError {}
