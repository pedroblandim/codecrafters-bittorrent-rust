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

            let contents = fs::read(file_path).expect(&format!("Could not read file {file_path}"));

            let decoded_value = BencodeTypes::decode(contents.clone());

            let decoded_json = decoded_value.serialize_to_json();

            let info = decoded_json.get("info").unwrap();

            let deserialized_info = BencodeTypes::deserialize_from_json(info).unwrap();

            // let mut encoded_info = b"".to_vec();
            // encoded_info.append(&mut BencodeTypes::encode(&deserialized_info));
            // encoded_info.append(&mut b"".to_vec());

            let encoded_info = BencodeTypes::encode(&deserialized_info);

            let info_hash = Sha1::digest(encoded_info.clone());

            println!(
                "Tracker URL: {}",
                decoded_json
                    .get("announce")
                    .unwrap()
                    .to_string()
                    .trim_matches('\"')
            );
            println!("Length: {}", info.get("length").unwrap());
            println!("Info Hash: {info_hash:x}");
        }
    }
}
