use std::fmt;

/// Error type for canonicalization operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalizationError {
    /// Invalid JSON input
    InvalidJson(String),
    /// Duplicate key found in object
    DuplicateKey(String),
    /// Other canonicalization errors
    Other(String),
}

impl fmt::Display for CanonicalizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CanonicalizationError::InvalidJson(msg) => write!(f, "Invalid JSON: {}", msg),
            CanonicalizationError::DuplicateKey(key) => write!(f, "Duplicate key: {}", key),
            CanonicalizationError::Other(msg) => write!(f, "Canonicalization error: {}", msg),
        }
    }
}

impl std::error::Error for CanonicalizationError {}
