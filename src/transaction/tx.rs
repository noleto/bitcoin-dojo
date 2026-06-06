use std::io::Read;

use crate::transaction::tx_input::TxInput;
use crate::transaction::tx_output::TxOutput;
use crate::utils::varint::decode_varint;
use std::error::Error;

#[derive(Clone)]
pub struct Tx {
    pub version: u32,
    pub tx_ins: Vec<TxInput>,
    pub tx_outs: Vec<TxOutput>,
}

impl Tx {
    pub fn new(version: u32, tx_ins: Vec<TxInput>, tx_outs: Vec<TxOutput>) -> Self {
        Self {
            version,
            tx_ins,
            tx_outs,
        }
    }

    // Parse the first 4 bytes of a transaction and interpret them as a little-endian 32-bit integer.
    pub fn parse<R: Read>(mut reader: R) -> Result<Self, Box<dyn Error>> {
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        let version = u32::from_le_bytes(buffer);

        let inputs = Self::parse_compact_list(reader.by_ref(), TxInput::parse)?;
        let outputs = Self::parse_compact_list(reader.by_ref(), TxOutput::parse)?;

        Ok(Self::new(version, inputs, outputs))
    }

    pub fn parse_compact_list<R: Read, F, U>(
        reader: &mut R,
        mut parser: F,
    ) -> Result<Vec<U>, Box<dyn Error>>
    where
        F: FnMut(&mut R) -> Result<U, Box<dyn Error>>,
    {
        let count = decode_varint(reader)? as usize;
        (0..count).map(|_| parser(reader)).collect()
    }
}
