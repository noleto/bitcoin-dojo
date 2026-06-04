use std::io::Read;

#[derive(Clone)]
pub struct Tx {
    pub version: u32,
}

impl Tx {
    pub fn new(version: u32) -> Self {
        Self { version }
    }

    // Parse the first 4 bytes of a transaction and interpret them as a little-endian 32-bit integer.
    pub fn parse<R: Read>(mut reader: R) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        Ok(Self::new(u32::from_le_bytes(buffer)))
    }
}
