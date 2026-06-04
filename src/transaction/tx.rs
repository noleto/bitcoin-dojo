use std::io::Read;

use crate::transaction::tx_input::TxInput;
use crate::utils::varint::decode_varint;

#[derive(Clone)]
pub struct Tx {
    pub version: u32,
    pub tx_ins: Vec<TxInput>,
}

impl Tx {
    pub fn new(version: u32, tx_ins: Vec<TxInput>) -> Self {
        Self { version, tx_ins }
    }

    // Parse the first 4 bytes of a transaction and interpret them as a little-endian 32-bit integer.
    pub fn parse<R: Read>(mut reader: R) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        let version = u32::from_le_bytes(buffer);

        let input_size = decode_varint(&mut reader)? as usize;

        let mut inputs = Vec::with_capacity(input_size);
        for _ in 0..input_size {
            inputs.push(TxInput::parse(reader.by_ref())?);
        }

        Ok(Self::new(version, inputs))
    }
}
