use crate::chunks::{tEXt, Chunk, ChunkBasicInfo, Unknown, IDAT, IEND, IHDR, PLTE};

pub struct ChunkIterator<'a> {
    i: usize,
    bytes: &'a Vec<u8>,
}

impl<'a> ChunkIterator<'a> {
    pub fn new(bytes: &'a Vec<u8>) -> Self {
        check_if_png(&bytes);
        ChunkIterator { bytes, i: 8 }
    }
}

impl<'a> Iterator for ChunkIterator<'a> {
    type Item = Box<dyn Chunk>;

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

        let info = ChunkBasicInfo {
            CRC: [0; 4], //TODO: calculate CRC
            data_length: chunk_length,
            type_str: String::from(chunk_type_str),
            type_bytes: chunk_type,
        };

        let chunk_type: Box<dyn Chunk> = match chunk_type_str {
            "IHDR" => Box::from(IHDR::new(self.i, &self.bytes, info)),
            "PLTE" => Box::from(PLTE::new(info)),
            "IDAT" => Box::from(IDAT::new(info, &self.bytes)),
            "IEND" => Box::from(IEND::new(info)),
            "tEXt" => Box::from(tEXt::new(self.i, info, self.bytes)),
            // "cHRM" => ,
            // "gAMA" => ,
            // "iCCP" => ,
            // "sBIT" => ,
            // "sRGB" => ,
            // "hIST" => ,
            // "tRNS" => ,
            // "pHYs" => ,
            // "sPLT" => ,
            // "tIME" => ,
            // "iTXt" => ,
            // "zTXt" => ,
            _ => Box::from(Unknown::new(info)),
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
