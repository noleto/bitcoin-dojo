use crate::utils::varint::{decode_varint, encode_varint};
use std::error::Error;
use std::io::Read;

#[derive(Clone)]
pub struct TxInput {
    pub prev_tx_id: [u8; 32], // little endian
    pub prev_index: u32,
    pub script_sig: Vec<u8>, // Store the scriptSig as a Vec<u8> for now, we will parse it in a later track
    pub sequence: u32,
}

impl TxInput {
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let mut prev_tx_id = [0u8; 32];
        reader.read_exact(&mut prev_tx_id)?;

        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        let prev_index = u32::from_le_bytes(buffer);

        let script_size = decode_varint(reader)? as usize;
        let mut script_sig = vec![0u8; script_size];
        reader.read_exact(&mut script_sig)?;

        reader.read_exact(&mut buffer)?;
        let sequence = u32::from_le_bytes(buffer);
        Ok(Self {
            prev_tx_id,
            prev_index,
            script_sig,
            sequence,
        })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let script_sig_compact_size = encode_varint(self.script_sig.len() as u64);
        let mut buffer = Vec::with_capacity(
            self.prev_tx_id.len() + script_sig_compact_size.len() + self.script_sig.len() + 8,
        );

        //push prev_tx_id 32 bytes Little-Endian
        buffer.extend(&self.prev_tx_id);

        //push prev_index 4 bytes	Little-Endian
        buffer.extend(self.prev_index.to_le_bytes());

        //push script_sig Compact Size
        buffer.extend(script_sig_compact_size);

        //push ScriptSig
        buffer.extend(&self.script_sig);

        //push sequence 4 bytes	Little-Endian
        buffer.extend(self.sequence.to_le_bytes());

        buffer
    }
}
