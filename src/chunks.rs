pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

/// Chunk can be any length, so we cannot assume it is x number of bytes
#[derive(Debug)]
pub struct Chunk {
    pub length: u32,
    pub data_type: u32,
    pub data: usize,
    pub CRC: u32,
}

#[derive(Debug)]
pub struct IHDR {
    pub width: [u8; 4],
    pub height: [u8; 4],
    pub bit_depth: u8,
    pub color_type: u8,
    pub compression_method: u8,
    pub filter_method: u8,
    pub interlace_method: u8,
}

pub struct PLTE {
    pub palette: [Color; 256],
}

pub struct IDAT {
    pub image_data: [u8; 32],
}
