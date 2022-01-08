use std::fs;

fn main() {
    let file_bytes = fs::read("./image.png").expect("Cannot read file");

    check_if_png(&file_bytes);

    let chunk_iter = ChunkIterator::new(&file_bytes);

    for chunk in chunk_iter {
        println!("{:?}", chunk);
    }
}

#[derive(Debug)]
struct Chunk {
    pub length: u32,
    pub data_type: u32,
    pub data: usize,
    pub CRC: u32,
}

struct ChunkIterator<'a> {
    pub bytes: &'a Vec<u8>,
}
impl<'a> ChunkIterator<'a> {
    fn new(bytes: &'a Vec<u8>) -> Self {
        ChunkIterator { bytes }
    }
}

impl<'a> Iterator for ChunkIterator<'a> {
    type Item = Chunk;

    fn next(&mut self) -> Option<Self::Item> {
        let length = 0;
        let data_type = 0;
        let data = 0;
        let CRC = 0;

        let chunk = Chunk {
            length,
            data_type,
            data,
            CRC,
        };
        Some(chunk)
    }
}

fn check_if_png(file_bytes: &Vec<u8>) {
    let png_header = [137, 80, 78, 71, 13, 10, 26, 10];
    let header = &file_bytes[0..8];

    if header != png_header {
        panic!("First 8 bytes do not conform to png header bytes")
    }
}
