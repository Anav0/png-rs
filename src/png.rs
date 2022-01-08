/// Chunk can be any length, so we cannot assume it is x number of bytes
#[derive(Debug)]
pub struct Chunk {
    pub length: u32,
    pub data_type: u32,
    pub data: usize,
    pub CRC: u32,
}

pub enum ChunkTypes {
    IHDR(IHDR),
    IDAT(IDAT),
    PLTE(PLTE),
    IEND,
    Text(String),
    Unknown(String, usize, Vec<u8>),
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

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct PLTE {
    pub palette: [Color; 256],
}

pub struct IDAT {
    pub image_data: [u8; 32],
}

pub struct ChunkIterator<'a> {
    i: usize,
    bytes: &'a Vec<u8>,
}

impl<'a> ChunkIterator<'a> {
    pub fn new(bytes: &'a Vec<u8>) -> Self {
        check_if_png(&bytes);
        ChunkIterator { bytes, i: 8 }
    }

    pub(crate) fn parse_ihdr(&self) -> ChunkTypes {
        let start = self.i + 8; //Skipping chunk length and type
        let width: [u8; 4] = self.bytes[start..start + 4].try_into().unwrap();
        let height: [u8; 4] = self.bytes[start + 4..start + 8].try_into().unwrap();

        let chunk = IHDR {
            width,
            height,
            bit_depth: self.bytes[start + 8],
            color_type: self.bytes[start + 9],
            compression_method: self.bytes[start + 10],
            filter_method: self.bytes[start + 11],
            interlace_method: self.bytes[start + 12],
        };

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

    pub(crate) fn parse_text(&self, data_length: usize) -> ChunkTypes {
        let start = self.i + 8; //Skipping chunk length and type
        let text = std::str::from_utf8(&self.bytes[start..start + data_length])
            .expect("Failed to read text from chunk");

        ChunkTypes::Text(String::from(text))
    }
}

impl<'a> Iterator for ChunkIterator<'a> {
    type Item = ChunkTypes;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i + 8 > self.bytes.len() {
            return None;
        }
        let chunk_length: [u8; 4] = self.bytes[self.i..self.i + 4]
            .try_into()
            .expect("Failed to read chunk data length");
        let chunk_type: [u8; 4] = self.bytes[self.i + 4..self.i + 8]
            .try_into()
            .expect("Failed to read chunk type");
        let chunk_length_in_bytes = u32::from_be_bytes(chunk_length);

        let chunk_type_str = std::str::from_utf8(&chunk_type).expect("Failed to read chunk type");

        //Chunk length + chunk type + data length + CRC
        let chunk_total_length = (4 + 4 + chunk_length_in_bytes + 4) as usize;

        let chunk_type = match chunk_type_str.to_lowercase().as_str() {
            "ihdr" => self.parse_ihdr(),
            "plte" => self.parse_plte(),
            "idat" => self.parse_Idat(),
            "iend" => ChunkTypes::IEND,
            "text" => self.parse_text(chunk_length_in_bytes as usize),
            _ => ChunkTypes::Unknown(
                String::from(chunk_type_str),
                chunk_length_in_bytes as usize,
                self.bytes[self.i..self.i + chunk_total_length].to_vec(),
            ),
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
