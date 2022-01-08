use std::{
    borrow::{Borrow, BorrowMut},
    fs,
    ops::Range,
};

fn main() {
    let file_bytes = fs::read("./image.png").expect("Cannot read file");

    check_if_png(&file_bytes);

    let chunk_iter = ChunkIterator::new(&file_bytes);

    for chunk in chunk_iter {}
}

/// Chunk can be any length, so we cannot assume it is x number of bytes
#[derive(Debug)]
struct Chunk {
    pub length: u32,
    pub data_type: u32,
    pub data: usize,
    pub CRC: u32,
}

enum ChunkTypes {
    IHDR(IHDR),
    IDAT(IDAT),
    PLTE(PLTE),
    IEND(IEND),
    Unknown,
}

#[derive(Debug)]
struct IHDR {
    pub width: [u8; 4],
    pub height: [u8; 4],
    pub bit_depth: u8,
    pub color_type: u8,
    pub compression_method: u8,
    pub filter_method: u8,
    pub interlace_method: u8,
}

struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

struct PLTE {
    pub palette: [Color; 256],
}
struct IDAT {
    pub image_data: [u8; 32],
}
struct IEND {}

struct ChunkIterator<'a> {
    i: usize,
    bytes: &'a Vec<u8>,
}

impl<'a> ChunkIterator<'a> {
    fn new(bytes: &'a Vec<u8>) -> Self {
        ChunkIterator { bytes, i: 8 }
    }

    pub(crate) fn parse_ihdr(&self) -> ChunkTypes {
        let start = self.i + 8; //Skipping chunk length and type
        let width: [u8; 4] = self.bytes[start..start + 4].try_into().unwrap();
        let height: [u8; 4] = self.bytes[start + 4..start + 8].try_into().unwrap();

        println!("Width: {:?} = {}px", width, u32::from_be_bytes(width));
        println!("Height: {:?} = {}px", height, u32::from_be_bytes(height));

        let chunk = IHDR {
            width,
            height,
            bit_depth: self.bytes[start + 8],
            color_type: self.bytes[start + 9],
            compression_method: self.bytes[start + 10],
            filter_method: self.bytes[start + 11],
            interlace_method: self.bytes[start + 12],
        };
        println!("Bit depth: {}", chunk.bit_depth);
        println!("Color type: {}", chunk.color_type);
        println!("Compression method: {}", chunk.compression_method);
        println!("Filter method: {}", chunk.filter_method);
        println!("Interlace method: {}", chunk.interlace_method);
        //TODO: Add CRC parsing

        ChunkTypes::IHDR(chunk)
    }

    pub(crate) fn parse_plte(&self) -> ChunkTypes {
        return ChunkTypes::IDAT(IDAT {
            image_data: [0; 32],
        });
    }

    pub(crate) fn parse_Idat(&self) -> ChunkTypes {
        return ChunkTypes::IDAT(IDAT {
            image_data: [0; 32],
        });
    }

    pub(crate) fn parse_iend(&self) -> ChunkTypes {
        return ChunkTypes::IDAT(IDAT {
            image_data: [0; 32],
        });
    }
}

impl<'a> Iterator for ChunkIterator<'a> {
    type Item = ChunkTypes;

    fn next(&mut self) -> Option<Self::Item> {
        println!("============");
        if self.i + 8 > self.bytes.len() {
            return None;
        }
        let chunk_length: [u8; 4] = self.bytes[self.i..self.i + 4].try_into().unwrap();
        let chunk_type: [u8; 4] = self.bytes[self.i + 4..self.i + 8].try_into().unwrap();
        let chunk_length_in_bytes = u32::from_be_bytes(chunk_length);
        println!(
            "Chunk data length: {:?} = {}b",
            chunk_length, chunk_length_in_bytes
        );

        let chunk_type_str = std::str::from_utf8(&chunk_type).expect("Failed to read chunk type");
        println!("Chunk type bytes: {:?} = {}", chunk_type, chunk_type_str);

        //Chunk length + chunk type + data length + CRC
        let chunk_total_length = (4 + 4 + chunk_length_in_bytes + 4) as usize;

        println!("Total chunk size: {}b", chunk_total_length);

        let chunk_type = match chunk_type_str {
            "IHDR" => self.parse_ihdr(),
            "PLTE" => self.parse_plte(),
            "IDAT" => self.parse_Idat(),
            "IEND" => self.parse_iend(),
            _ => ChunkTypes::Unknown,
        };

        self.i += chunk_total_length;

        Some(chunk_type)
    }
}

fn check_if_png(file_bytes: &Vec<u8>) {
    let png_header = [137, 80, 78, 71, 13, 10, 26, 10];
    let header = &file_bytes[0..=7];

    if header != png_header {
        panic!("First 8 bytes do not conform to png header bytes")
    }
}
