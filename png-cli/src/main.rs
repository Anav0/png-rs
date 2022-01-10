use png_core::{
    encoders::{CustomChunkEncoder, Encoder, EndOfFileEncoder},
    ChunkIterator,
};
use std::{env, fs};

enum Encoding {
    AtTheEnd,
    InCustomChunk,
}

struct Parameters {
    decode: Option<Encoding>,
    print_info: bool,
    is_encoding: bool,
    output_filename: String,
}
impl Default for Parameters {
    fn default() -> Self {
        Self {
            decode: None,
            is_encoding: false,
            print_info: false,
            output_filename: String::from("output.png"),
        }
    }
}

fn main() {
    let mut parameters = Parameters::default();
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let mut message = String::new();

    let mut i = 0;
    for arg in &args {
        match arg.as_str() {
            "-h" => {
                print_help();
                return;
            }
            "-o" => {
                parameters.output_filename = args[i + 1].clone() + ".png";
            }
            "-i" => {
                parameters.print_info = true;
            }
            "-d" => {
                parameters.decode = match args[i + 1].as_str() {
                    "end" => Some(Encoding::AtTheEnd),
                    "chunk" => Some(Encoding::InCustomChunk),
                    _ => None,
                };
                parameters.is_encoding = false;
            }
            "-e" => {
                parameters.decode = match args[i + 1].as_str() {
                    "end" => Some(Encoding::AtTheEnd),
                    "chunk" => Some(Encoding::InCustomChunk),
                    _ => None,
                };
                parameters.is_encoding = true;
            }
            "-m" => {
                message = args[i + 1].clone();
            }
            _ => {}
        }
        i += 1;
    }

    let mut file_bytes = fs::read(file_path).expect("Cannot read png file");

    if parameters.print_info {
        let chunk_iter = ChunkIterator::new(&file_bytes);

        for chunk in chunk_iter {
            println!("{}", chunk);
        }
    }

    if parameters.is_encoding {
        let encoding_method = parameters.decode.unwrap();

        match encoding_method {
            Encoding::AtTheEnd => EndOfFileEncoder::new().encode(
                &parameters.output_filename,
                &message,
                &mut file_bytes,
            ),
            Encoding::InCustomChunk => CustomChunkEncoder::new().encode(
                &parameters.output_filename,
                &message,
                &mut file_bytes,
            ),
        };
    }
}

fn print_help() {
    println!("PNG file format explorer");
    println!("-h - displays help");
    println!("-i - prints info about PNG file");
    println!("-d end | chunk - decoding message from image");
    println!("-e end | chunk - encodes message");
    println!("-m <message> - message to encode");
    println!("-o <filename> - output filename");
}
