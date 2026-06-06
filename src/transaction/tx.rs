use std::io::Read;

use crate::transaction::tx_input::TxInput;
use crate::transaction::tx_output::TxOutput;
use crate::utils::varint::{decode_varint, encode_varint};
use std::error::Error;

#[derive(Clone)]
pub struct Tx {
    pub version: u32,
    pub tx_ins: Vec<TxInput>,
    pub tx_outs: Vec<TxOutput>,
    pub locktime: u32,
}

impl Tx {
    pub fn new(version: u32, tx_ins: Vec<TxInput>, tx_outs: Vec<TxOutput>, locktime: u32) -> Self {
        Self {
            version,
            tx_ins,
            tx_outs,
            locktime,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        //push Version 4 bytes	Little-Endian
        buffer.extend(self.version.to_le_bytes());

        //push Input Count Compact Size
        buffer.extend(encode_varint(self.tx_ins.len() as u64));

        //push all inputs
        for input in &self.tx_ins {
            buffer.extend(input.serialize());
        }

        //push Output Count Compact Size
        buffer.extend(encode_varint(self.tx_outs.len() as u64));

        //push all outputs
        for output in &self.tx_outs {
            buffer.extend(output.serialize());
        }

        //push Locktime 4 bytes	Little-Endian
        buffer.extend(self.locktime.to_le_bytes());

        buffer
    }

    // Parse the first 4 bytes of a transaction and interpret them as a little-endian 32-bit integer.
    pub fn parse<R: Read>(mut reader: R) -> Result<Self, Box<dyn Error>> {
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        let version = u32::from_le_bytes(buffer);

        let inputs = Self::parse_compact_list(reader.by_ref(), TxInput::parse)?;
        let outputs = Self::parse_compact_list(reader.by_ref(), TxOutput::parse)?;

        //locktime
        reader.read_exact(&mut buffer)?;
        let locktime = u32::from_le_bytes(buffer);

        Ok(Self::new(version, inputs, outputs, locktime))
    }

    pub fn parse_compact_list<R: Read, F, U>(
        reader: &mut R,
        parser: F,
    ) -> Result<Vec<U>, Box<dyn Error>>
    where
        F: Fn(&mut R) -> Result<U, Box<dyn Error>>,
    {
        let count = decode_varint(reader)? as usize;
        (0..count).map(|_| parser(reader)).collect()
    }
}
