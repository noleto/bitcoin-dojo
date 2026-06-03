use std::io::{Error, Read};

#[derive(PartialEq)]
enum CompactSizeMarker {
    OxFC,
    OxFD,
    OxFE,
    OxFF,
}

impl CompactSizeMarker {
    fn len(&self) -> usize {
        match self {
            Self::OxFC => 1,
            Self::OxFD => 3,
            Self::OxFE => 5,
            Self::OxFF => 9,
        }
    }

    fn byte(&self) -> u8 {
        match self {
            Self::OxFC => 0xFC,
            Self::OxFD => 0xFD,
            Self::OxFE => 0xFE,
            Self::OxFF => 0xFF,
        }
    }

    fn value_to_marker(value: u64) -> CompactSizeMarker {
        match value {
            0..0xFD => CompactSizeMarker::OxFC,
            0xFD..0x10000 => CompactSizeMarker::OxFD,
            0x10000..0x100000000 => CompactSizeMarker::OxFE,
            _ => CompactSizeMarker::OxFF,
        }
    }

    pub fn byte_to_marker(byte_mark: u8) -> CompactSizeMarker {
        match byte_mark {
            0..0xFD => CompactSizeMarker::OxFC,
            0xFD => CompactSizeMarker::OxFD,
            0xFE => CompactSizeMarker::OxFE,
            0xFF => CompactSizeMarker::OxFF,
        }
    }
}

/// Variable-length integer encoding and decoding functions
///
/// Varints encode integers from 0 to 2^64 - 1 using variable-length encoding:
/// - 0x00 to 0xFC: stored as single byte
/// - 0xFD: followed by 2-byte little-endian value (253 to 65535)
/// - 0xFE: followed by 4-byte little-endian value (65536 to 4294967295)
/// - 0xFF: followed by 8-byte little-endian value (4294967296 to 18446744073709551615)
/// Encode a u64 value as a varint
pub fn encode_varint(value: u64) -> Vec<u8> {
    let compact_size = CompactSizeMarker::value_to_marker(value);
    if compact_size == CompactSizeMarker::OxFC {
        return vec![value as u8];
    } else {
        let mut buffer = vec![0u8; compact_size.len()];
        buffer[0] = compact_size.byte();
        buffer[1..].copy_from_slice(&value.to_le_bytes()[..(compact_size.len() - 1)]);
        buffer
    }
}

/// Reads a varint from a reader
pub fn decode_varint<R: Read>(reader: &mut R) -> Result<u64, Error> {
    let mut first_byte = [0u8; 1];
    reader.read_exact(&mut first_byte)?;

    let compact_size = CompactSizeMarker::byte_to_marker(first_byte[0]);

    if compact_size == CompactSizeMarker::OxFC {
        return Ok(first_byte[0] as u64);
    }

    let value_len = compact_size.len() - 1;
    let mut value_bytes = [0u8; 8];
    reader.read_exact(&mut value_bytes[..value_len])?;
    let decoded_value = u64::from_le_bytes(value_bytes);
    if varint_length(decoded_value) != compact_size.len() {
        return Err(Error::other("Wrong encoding, should use less bytes"));
    }
    Ok(decoded_value)
}

/// Get the encoded length of a varint for a given value
/// This function is required by BitcoinDojo evaluator (tests) so we need to keep it
pub fn varint_length(value: u64) -> usize {
    CompactSizeMarker::value_to_marker(value).len()
}
