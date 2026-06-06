use crate::utils::varint::decode_varint;
use std::error::Error;
use std::io::Read;

#[derive(Clone, Debug)]
pub struct TxOutput {
    pub amount: u64,
    pub script_pubkey: Vec<u8>, // Store the script_pubkey as a Vec<u8> for now, we will parse it in a later track
}

impl TxOutput {
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let mut buffer = [0u8; 8];
        reader.read_exact(&mut buffer)?;
        let amount = u64::from_le_bytes(buffer);

        let script_size = decode_varint(reader.by_ref())?;
        let mut script_pubkey = vec![0u8; script_size as usize];
        reader.read_exact(&mut script_pubkey)?;

        Ok(Self {
            amount,
            script_pubkey,
        })
    }
}
