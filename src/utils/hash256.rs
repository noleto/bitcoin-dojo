use sha2::{Digest, Sha256};

/// Performs a double SHA-256 hash (hash256 = sha256(sha256(data)))
/// This is commonly used in Bitcoin for additional security
///
/// # Returns
/// A 32-byte array containing the double SHA-256 hash
pub fn hash256(data: &[u8]) -> [u8; 32] {
    Sha256::digest(Sha256::digest(data)).into()
}
