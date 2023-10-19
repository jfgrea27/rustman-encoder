use std::env;
mod huffman;
use std::process;
mod rustman_encoder_helper {
    pub struct Config {
        pub action: String,
        pub input_path: String,
        pub output_path: String,
    }

    impl Config {
        pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
            args.next();

            let action = match args.next() {
                Some(arg) => arg,
                None => return Err("No action supplied."),
            };
            let input_path = match args.next() {
                Some(arg) => arg,
                None => return Err("No input file path supplied"),
            };

            let output_path = match args.next() {
                Some(arg) => arg,
                None => return Err("No output file path supplied"),
            };

            Ok(Config {
                action,
                input_path,
                output_path,
            })
        }
    }
}

pub fn main() {
    use crate::huffman::huffman::{decode, encode};

    use rustman_encoder_helper::Config;
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    match config.action.as_str() {
        "encode" => encode(&config.input_path, &config.output_path),
        "decode" => decode(&config.input_path, &config.output_path),
        _ => panic!("Invalid action - please choose encode/decode"),
    };
}
