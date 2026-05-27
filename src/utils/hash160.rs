use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

/// Performs HASH160 operation: RIPEMD160(SHA256(input))
/// This is commonly used in Bitcoin for creating addresses from public keys

/// # Returns
/// A 20-byte array containing the HASH160 result
pub fn hash160(input: &[u8]) -> [u8; 20] {
    Ripemd160::digest(Sha256::digest(input)).into()
}
