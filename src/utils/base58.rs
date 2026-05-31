use num_bigint::BigUint;
use num_traits::{Euclid, ToPrimitive, Zero};

use crate::utils::hash256::hash256;

const BASE58_ALPHABET: &'static [u8] =
    b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
/// Encodes a byte array into Base58 format
/// Base58 is used in Bitcoin to encode addresses and other data
/// It uses an alphabet that excludes confusing characters (0, O, I, l)
pub fn encode_base58(input: &[u8]) -> String {
    if input.is_empty() {
        return String::new();
    }

    let leading_zeros = input.iter().take_while(|e| e.is_zero()).count();

    let mut quotient = BigUint::from_bytes_be(&input[leading_zeros..]);

    let mut buffer = Vec::with_capacity(input.len() * 2);
    let base58_biguint = BigUint::from(58u32);
    while !quotient.is_zero() {
        let (new_quotient, remainder) = quotient.div_rem_euclid(&base58_biguint);
        let base58_index = remainder.to_usize().expect("Remainder too large!");

        buffer.push(BASE58_ALPHABET[base58_index]);

        quotient = new_quotient;
    }

    //leading zeros are encoded with "1" (0x31 byte)
    buffer.extend(vec![b'1'; leading_zeros]);

    buffer.reverse();

    String::from_utf8(buffer).unwrap()
}

/// Decodes a Base58 encoded string back to bytes
/// Returns an error if the input contains invalid Base58 characters
pub fn decode_base58(input: &str) -> Result<Vec<u8>, &'static str> {
    if input.is_empty() {
        return Ok(Vec::new());
    }

    let leading_ones = input.chars().take_while(|&c| c == '1').count();

    let mut big_number = BigUint::ZERO;
    let base58_biguint = BigUint::from(58u32);
    for character in input[leading_ones..].chars() {
        let position = BASE58_ALPHABET
            .iter()
            .position(|&a| a == character as u8)
            .ok_or("Invalid character: not in Base58 alphabet")?;
        //use Horner's method that factors out the base
        big_number = big_number * &base58_biguint + position
    }
    let mut result = vec![0u8; leading_ones];
    if big_number != BigUint::ZERO {
        result.extend(big_number.to_bytes_be());
    }
    Ok(result)
}

/// Encodes a byte array into Base58Check format
/// Base58Check adds a 4-byte checksum to the data before encoding
/// This is used in Bitcoin for addresses, private keys, and other critical data
/// The checksum is the first 4 bytes of SHA256(SHA256(data))
pub fn encode_base58_check(input: &[u8]) -> String {
    if input.is_empty() {
        return String::new();
    }
    let checksum_bytes = hash256(input);
    let mut payload = Vec::with_capacity(input.len() + 4);
    payload.extend_from_slice(input);
    payload.extend_from_slice(&checksum_bytes[..4]);
    encode_base58(&payload)
}

/// Decodes a Base58Check encoded string back to the original data
/// Base58Check includes a 4-byte checksum that is verified during decoding
/// Returns an error if the input is invalid or the checksum doesn't match
pub fn decode_base58_check(input: &str) -> Result<Vec<u8>, &'static str> {
    if input.is_empty() {
        return Ok(Vec::new());
    }
    let mut decoded = decode_base58(input)?;
    let checksum_start = decoded
        .len()
        .checked_sub(4)
        .ok_or("Base58Check data too short: must be at least 4 bytes")?;
    if &hash256(&decoded[..checksum_start])[..4] != &decoded[checksum_start..] {
        return Err("Base58Check checksum verification failed");
    }
    // Shirink to keep only the useful payload
    decoded.truncate(checksum_start);

    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_base58_fofo() {
        assert_eq!(
            encode_base58(&hex::decode("f0f0").expect("Decoding failed")),
            "KLT"
        );
    }

    #[test]
    fn decode_base58_fofo() {
        assert_eq!(
            decode_base58("KLT").unwrap(),
            hex::decode("f0f0").expect("Decoding failed")
        );
    }
    #[test]
    fn encode_base58check_fofo() {
        assert_eq!(
            encode_base58_check(&hex::decode("00F0F0").expect("Decoding failed")),
            "134yvs61PS"
        );
    }
}
