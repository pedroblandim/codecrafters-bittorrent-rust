use std::fs;
use std::{env, str::FromStr};

use sha1::{Digest, Sha1};

use crate::{bencode::bencode_types::BencodeTypes, commands::Commands};

mod bencode;
mod commands;

// Usage: your_program.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command_string = match args.get(1) {
        Some(s) => s,
        None => "info",
    };

    let Ok(command) = Commands::from_str(command_string) else {
        println!("unknown command: {}", args[1]);
        return ();
    };

    match command {
        Commands::Decode => {
            let encoded_value = &args[2];

            let decoded_value = BencodeTypes::decode(encoded_value.to_owned().into_bytes());

            let json_value = decoded_value.serialize_to_json();

            println!("{}", json_value.to_string());
        }
        Commands::Info => {
            let file_path = match args.get(2) {
                Some(p) => p,
                None => "sample.torrent",
            };

            let content = fs::read(file_path).expect(&format!("Could not read file {file_path}"));

            let content_decoded = BencodeTypes::decode(content.clone());

            let info_encoded;
            let announce;
            let length;

            match content_decoded {
                BencodeTypes::Dictionary(map) => {
                    let info_decoded = map.get(&b"info".to_vec()).unwrap().clone();
                    info_encoded = BencodeTypes::encode(&info_decoded);

                    announce = match map.get(&b"announce".to_vec()).unwrap().clone() {
                        BencodeTypes::ByteString(string) => string,
                        _ => panic!("error"),
                    };

                    length = match info_decoded {
                        BencodeTypes::Dictionary(map) => {
                            match map.get(&b"length".to_vec()).unwrap().clone() {
                                BencodeTypes::Integer(number) => number,
                                _ => panic!("error"),
                            }
                        }
                        _ => panic!("error"),
                    };
                }
                _ => panic!("error"),
            };

            let info_encoded_hash = Sha1::digest(info_encoded.clone());

            println!("Tracker URL: {}", String::from_utf8_lossy(&announce));
            println!("Length: {}", length);
            println!("Info Hash: {info_encoded_hash:x}");
        }
    }
}
