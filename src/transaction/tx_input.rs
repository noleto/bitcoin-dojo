use crate::utils::varint::decode_varint;
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
    pub fn parse<R: Read>(mut reader: R) -> Result<Self, Box<dyn Error>> {
        let mut prev_tx_id = [0u8; 32];
        reader.read_exact(&mut prev_tx_id)?;

        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        let prev_index = u32::from_le_bytes(buffer);

        let script_size = decode_varint(&mut reader)? as usize;
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
}
