use std::fmt::Display;

use super::get_basic_data_str;

use super::{Chunk, ChunkBasicInfo};

pub struct tEXt {
    info: ChunkBasicInfo,
    text: String,
}

impl Display for tEXt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\t{}",
            get_basic_data_str("tEXt", &self.info),
            self.text
        )
    }
}

impl tEXt {
    pub fn new(i: usize, info: ChunkBasicInfo, bytes: &Vec<u8>) -> Self {
        let chunk_length_in_bytes = u32::from_be_bytes(info.data_length);
        let start = i + 8; //Skipping chunk length and type
        let text = std::str::from_utf8(&bytes[start..start + chunk_length_in_bytes as usize])
            .expect("Failed to read text from chunk");

        Self {
            info,
            text: String::from(text),
        }
    }
}

impl Chunk for tEXt {
    fn get_basic_info(&self) -> &ChunkBasicInfo {
        &self.info
    }
}
