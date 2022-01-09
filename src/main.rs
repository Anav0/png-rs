use png::{
    encoders::{Encoder, EndOfFileEncoder},
    ChunkIterator,
};
use std::{env, fs, ptr::NonNull, str::EncodeUtf16};

mod png;

enum Encoding {
    AtTheEnd,
    InCustomChunk,
}

struct Parameters {
    decode: Option<Encoding>,
    is_encoding: bool,
}
impl Default for Parameters {
    fn default() -> Self {
        Self {
            decode: None,
            is_encoding: false,
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
            "-d" => {
                parameters.decode = match args[i + 1].as_str() {
                    "0" => Some(Encoding::AtTheEnd),
                    "1" => Some(Encoding::InCustomChunk),
                    _ => None,
                };
                parameters.is_encoding = false;
            }
            "-e" => {
                parameters.decode = match args[i + 1].as_str() {
                    "0" => Some(Encoding::AtTheEnd),
                    "1" => Some(Encoding::InCustomChunk),
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

    let chunk_iter = ChunkIterator::new(&file_bytes);

    for chunk in chunk_iter {
        println!("{}", chunk);
    }

    if parameters.is_encoding {
        let encoding_method = parameters.decode.unwrap();

        let file_name = "encoded.png";

        match encoding_method {
            Encoding::AtTheEnd => {
                EndOfFileEncoder::new().encode(&file_name, &message, &mut file_bytes)
            }
            Encoding::InCustomChunk => {
                EndOfFileEncoder::new().encode(&file_name, &message, &mut file_bytes)
            }
        };
    }
}
