use std::{env, fs};

use png::ChunkIterator;

use crate::png::ChunkTypes;

mod png;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    for arg in &args {
        match arg.as_str() {
            "-s" => {}
            "-h" => {}
            _ => {}
        }
    }

    let file_bytes = fs::read(file_path).expect("Cannot read file");

    let chunk_iter = ChunkIterator::new(&file_bytes);

    for chunk in chunk_iter {
        match chunk {
            ChunkTypes::IHDR(data) => {
                println! {"IHDR:"}
                println!(
                    "\tSize: {}x{}px",
                    u32::from_be_bytes(data.width),
                    u32::from_be_bytes(data.height)
                );
                println!("\tBit depth: {}", data.bit_depth);
                println!("\tColor type: {}", data.color_type);
                println!("\tCompression method: {}", data.compression_method);
                println!("\tFilter method: {}", data.filter_method);
                println!("\tInterlace method: {}", data.interlace_method);
            }
            ChunkTypes::IDAT(data) => {
                println!("IDAT:");
            }
            ChunkTypes::PLTE(data) => {
                println!("PLTE:");
            }
            ChunkTypes::IEND => {
                println!("IEND:");
            }
            ChunkTypes::Text(text) => {
                println!("Text:");
                println!("\t{}", text)
            }
            ChunkTypes::Unknown(chunk_type_str, chunk_data_size, bytes) => {
                println!("Unknown chunk:");
                println!("\t Type: {}", chunk_type_str);
                println!("\t Data size: {}", chunk_data_size);
                println!("\t Bytes: {:?}", bytes);
            }
        }
    }
}
