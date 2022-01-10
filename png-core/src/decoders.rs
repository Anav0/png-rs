use crate::ChunkIterator;

pub trait Decoder {
    fn decode(&self, bytes: &Vec<u8>) -> Option<String>;
}

pub struct EndOfFileDecoder;
impl EndOfFileDecoder {
    pub fn new() -> Self {
        Self {}
    }
}

impl Decoder for EndOfFileDecoder {
    fn decode(&self, bytes: &Vec<u8>) -> Option<String> {
        let chunk_iterator = ChunkIterator::new(bytes);

        let mut end_chunk_end_pos = 8;
        for chunk in chunk_iterator {
            let info = chunk.get_basic_info();

            let chunk_data_size = u32::from_be_bytes(info.data_length);

            end_chunk_end_pos += (4 * 3) + chunk_data_size;
        }

        if end_chunk_end_pos as usize >= bytes.len() {
            return None;
        }

        let mut bytes_after_end: Vec<u8> =
            Vec::with_capacity(bytes.len() - end_chunk_end_pos as usize);

        for i in end_chunk_end_pos as usize..bytes.len() {
            bytes_after_end.push(bytes[i]);
        }

        Some(
            String::from_utf8(bytes_after_end)
                .expect("Failed to create text from bytes after IEND chunk"),
        )
    }
}
