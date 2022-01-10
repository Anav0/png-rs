use std::fs;

use crate::ChunkIterator;

pub trait Encoder {
    fn encode(&self, file_name: &str, message: &str, bytes: &mut Vec<u8>);
}

pub struct EndOfFileEncoder;
impl EndOfFileEncoder {
    pub fn new() -> Self {
        Self {}
    }
}

impl Encoder for EndOfFileEncoder {
    fn encode(&self, file_name: &str, message: &str, bytes: &mut Vec<u8>) {
        let mut message_bytes = message.as_bytes().to_vec();

        bytes.append(&mut message_bytes);

        fs::write(file_name, bytes).expect("Failed to write file with message after IEND chunk");
    }
}

pub struct CustomChunkEncoder;

impl CustomChunkEncoder {
    pub fn new() -> Self {
        Self {}
    }
}

impl Encoder for CustomChunkEncoder {
    fn encode(&self, file_name: &str, message: &str, bytes: &mut Vec<u8>) {
        let chunk_iter = ChunkIterator::new(&bytes);

        let mut end_chunk_start_pos = 8; //4 bytes of standard PNG header
        for chunk in chunk_iter {
            let info = chunk.get_basic_info();

            if info.type_str == "IEND" {
                break;
            }

            let data_length_decimal = u32::from_be_bytes(info.data_length);

            end_chunk_start_pos += (4 * 3) + data_length_decimal;
        }

        let mut message_bytes = message.as_bytes().to_vec();
        let mut custom_chunk_bytes: Vec<u8> = Vec::with_capacity((4 * 3) + message_bytes.len());

        let message_length = message_bytes.len() as u32;

        custom_chunk_bytes.append(&mut message_length.to_be_bytes().to_vec()); //data length
        custom_chunk_bytes.append(&mut "sMSG".as_bytes().to_vec()); //type
        custom_chunk_bytes.append(&mut message_bytes); //data
        custom_chunk_bytes.append(&mut vec![0, 0, 0, 0]); //CRC

        let mut end_chunk_bytes = bytes.split_off(end_chunk_start_pos as usize);

        bytes.append(&mut custom_chunk_bytes);
        bytes.append(&mut end_chunk_bytes);

        fs::write(file_name, bytes).expect("Failed to write file with custom chunk");
    }
}
