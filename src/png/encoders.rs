use std::fs;

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
