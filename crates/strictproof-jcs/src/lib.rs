mod error;

pub use error::CanonicalizationError;

/// Canonicalize JSON input according to RFC 8785.
///
/// Takes raw bytes representing JSON and returns the canonical form
/// that can be used for deterministic hashing and verification.
pub fn canonicalize(input: &[u8]) -> Result<Vec<u8>, CanonicalizationError> {
    // TODO: Implement RFC 8785 canonicalization
    Ok(input.to_vec())
}
