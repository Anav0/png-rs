use png::ChunkIterator;
use std::{env, fs};

mod chunks;
mod png;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let file_bytes = fs::read(file_path).expect("Cannot read file");

    let chunk_iter = ChunkIterator::new(&file_bytes);

    for chunk in chunk_iter {
        println!("{}", chunk);
    }
}
