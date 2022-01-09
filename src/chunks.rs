use std::{fmt::Display, mem::Discriminant};

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

#[derive(Debug)]
pub struct IHDR {
    pub info: ChunkBasicInfo,
    pub width: [u8; 4],
    pub height: [u8; 4],
    pub bit_depth: u8,
    pub color_type: u8,
    pub compression_method: u8,
    pub filter_method: u8,
    pub interlace_method: u8,
}

impl IHDR {
    pub fn new(i: usize, bytes: &Vec<u8>, info: ChunkBasicInfo) -> Self {
        let start = i + 8; //Skipping chunk length and type
        let width: [u8; 4] = bytes[start..start + 4].try_into().unwrap();
        let height: [u8; 4] = bytes[start + 4..start + 8].try_into().unwrap();

        Self {
            info,
            width,
            height,
            bit_depth: bytes[start + 8],
            color_type: bytes[start + 9],
            compression_method: bytes[start + 10],
            filter_method: bytes[start + 11],
            interlace_method: bytes[start + 12],
        }
    }
}
impl Display for IHDR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\tSize: {}x{}px\n\tBit depth: {}\n\tColor type: {}\n\tCompression method: {}\n\tFilter method: {}\n\tInterlace method: {}",
            get_basic_data_str("IHDR", &self.info),
            u32::from_be_bytes(self.width),
            u32::from_be_bytes(self.height),
            self.bit_depth,
            self.color_type,
            self.compression_method,
            self.filter_method,
            self.interlace_method
        )
    }
}
impl Chunk for IHDR {
    fn get_basic_info(&self) -> &ChunkBasicInfo {
        &self.info
    }
}

pub struct PLTE {
    pub info: ChunkBasicInfo,
    pub palette: Vec<Color>,
}

impl PLTE {
    pub fn new(info: ChunkBasicInfo, i: usize, bytes: &Vec<u8>) -> Self {
        let mut palette: Vec<Color> = Vec::with_capacity(info.data_length.len());

        let base_index = i + 8; //Skipping length and type
        for j in 0..info.data_length.len() {
            palette[j].red = bytes[base_index + j];
            palette[j].green = bytes[base_index + j + 1];
            palette[j].blue = bytes[base_index + j + 2];
        }

        Self { info, palette }
    }
}

impl Chunk for PLTE {
    fn get_basic_info(&self) -> &ChunkBasicInfo {
        &self.info
    }
}
impl Display for PLTE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\t{:?}",
            get_basic_data_str("PLTE", &self.info),
            self.palette
        )
    }
}
pub struct IDAT {
    pub info: ChunkBasicInfo,
    pub image_data: [u8; 32],
}

impl IDAT {
    pub fn new(info: ChunkBasicInfo, bytes: &Vec<u8>) -> Self {
        Self {
            info,
            image_data: [0; 32], //TODO: parse image data
        }
    }
}

impl Chunk for IDAT {
    fn get_basic_info(&self) -> &ChunkBasicInfo {
        &self.info
    }
}
impl Display for IDAT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", get_basic_data_str("IDAT", &self.info))
    }
}
pub struct IEND {
    info: ChunkBasicInfo,
}

impl IEND {
    pub fn new(info: ChunkBasicInfo) -> Self {
        Self { info }
    }
}

impl Display for IEND {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IEND")
    }
}

impl Chunk for IEND {
    fn get_basic_info(&self) -> &ChunkBasicInfo {
        &self.info
    }
}

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

fn get_basic_data_str(header: &str, info: &ChunkBasicInfo) -> String {
    format!("{}:\n\tType:          {}\n\tData length:   {:?}\n\tBytes:         {:?}\n\tCRC:           {:?}\n",
    header,
    info.type_str,
    info.data_length,
    info.type_bytes,
    info.CRC)
}
