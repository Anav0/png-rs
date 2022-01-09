use std::{env, fs};

use png::ChunkIterator;

use crate::png::ChunkTypes;

mod chunks;
mod png;

enum PrintType {
    Simple,
    Extended,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut print_type = PrintType::Extended;
    let mut skip_bytes = false;

    for arg in &args {
        match arg.as_str() {
            "-s" => {
                print_type = PrintType::Simple;
            }
            "-b" => skip_bytes = true,
            _ => {}
        }
    }

    let file_bytes = fs::read(file_path).expect("Cannot read file");

    let chunk_iter = ChunkIterator::new(&file_bytes);

    let print_fn = match print_type {
        PrintType::Extended => print_extended,
        PrintType::Simple => print_simple,
    };

    for chunk in chunk_iter {
        print_fn(&chunk, skip_bytes);
    }

    fn print_simple(chunk: &ChunkTypes, skip_bytes: bool) {
        match chunk {
            ChunkTypes::IHDR(data) => {
                println! {"IHDR:"}
                println!(
                    "\tSize: {}x{}px",
                    u32::from_be_bytes(data.width),
                    u32::from_be_bytes(data.height)
                );
            }
            ChunkTypes::IEND => {
                println!("IEND:");
            }
            ChunkTypes::tEXt(text) => {
                println!("Text:");
                println!("\t{}", text)
            }
            _ => {}
        }
    }

    fn print_extended(chunk: &ChunkTypes, skip_bytes: bool) {
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
            ChunkTypes::tEXt(text) => {
                println!("Text:");
                println!("\t{}", text)
            }
            ChunkTypes::Unknown(chunk_type_str, chunk_data_size, bytes) => {
                println!("Unknown chunk:");
                println!("\t Type: {}", chunk_type_str);
                println!("\t Data size: {}", chunk_data_size);

                if skip_bytes == false {
                    println!("\t Bytes: {:?}", bytes);
                }
            }
            _ => {}
        }
    }
}
