use crate::chunks::{IDAT, IHDR, PLTE};

pub enum ChunkTypes {
    IHDR(IHDR),
    PLTE(PLTE),
    IDAT(IDAT),
    IEND,
    cHRM,
    gAMA,
    iCCP,
    sBIT,
    sRGB,
    hIST,
    tRNS,
    pHYs,
    sPLT,
    tIME,
    iTXt,
    tEXt(String),
    zTXt,
    Unknown(String, usize, Vec<u8>),
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

        ChunkTypes::tEXt(String::from(text))
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

        let chunk_type = match chunk_type_str {
            "IHDR" => self.parse_ihdr(),
            "PLTE" => self.parse_plte(),
            "IDAT" => self.parse_Idat(),
            "IEND" => ChunkTypes::IEND,
            "cHRM" => ChunkTypes::cHRM,
            "gAMA" => ChunkTypes::gAMA,
            "iCCP" => ChunkTypes::iCCP,
            "sBIT" => ChunkTypes::sBIT,
            "sRGB" => ChunkTypes::sRGB,
            "hIST" => ChunkTypes::hIST,
            "tRNS" => ChunkTypes::tRNS,
            "pHYs" => ChunkTypes::pHYs,
            "sPLT" => ChunkTypes::sPLT,
            "tIME" => ChunkTypes::tIME,
            "iTXt" => ChunkTypes::iTXt,
            "zTXt" => ChunkTypes::zTXt,
            "tEXt" => self.parse_text(chunk_length_in_bytes as usize),
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
