use std::fs;

use png::ChunkIterator;

mod png;

fn main() {
    let file_bytes = fs::read("./image.png").expect("Cannot read file");

    let chunk_iter = ChunkIterator::new(&file_bytes);

    for chunk in chunk_iter {}
}
