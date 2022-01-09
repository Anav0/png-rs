use std::fmt::Display;

mod ancillary;
mod critical;

pub use ancillary::tEXt;
pub use critical::{IDAT, IEND, IHDR, PLTE};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            red: Default::default(),
            green: Default::default(),
            blue: Default::default(),
        }
    }
}
/// Chunk can be any length, so we cannot assume it is x number of bytes
#[derive(Debug)]
pub struct ChunkBasicInfo {
    pub data_length: [u8; 4],
    pub CRC: [u8; 4],
    pub type_str: String,
    pub type_bytes: [u8; 4],
}

pub struct Unknown {
    info: ChunkBasicInfo,
}
impl Display for Unknown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", get_basic_data_str("Unknown", &self.info))
    }
}

impl Unknown {
    pub fn new(info: ChunkBasicInfo) -> Self {
        Self { info }
    }
}

impl Chunk for Unknown {
    fn get_basic_info(&self) -> &ChunkBasicInfo {
        &self.info
    }
}

pub trait Chunk: Display {
    fn get_basic_info(&self) -> &ChunkBasicInfo;
}

pub(crate) fn get_basic_data_str(header: &str, info: &ChunkBasicInfo) -> String {
    format!("{}:\n\tType:          {}\n\tData length:   {:?}\n\tBytes:         {:?}\n\tCRC:           {:?}\n",
    header,
    info.type_str,
    info.data_length,
    info.type_bytes,
    info.CRC)
}
