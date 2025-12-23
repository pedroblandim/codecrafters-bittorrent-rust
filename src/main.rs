use std::fs;
use std::{env, str::FromStr};

use crate::{
    commands::Commands,
    types::bencode_types::{BencodeTypes, ToJSON},
};

mod commands;
mod types;

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: Vec<u8>) -> serde_json::Value {
    let decoded = BencodeTypes::parse(encoded_value);

    decoded.serialize()
}

// Usage: your_program.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command_string = &args[1];

    let Ok(command) = Commands::from_str(command_string) else {
        println!("unknown command: {}", args[1]);
        return ();
    };

    match command {
        Commands::Decode => {
            let encoded_value = &args[2];
            let decoded_value = decode_bencoded_value(encoded_value.to_owned().into_bytes());
            println!("{}", decoded_value.to_string());
        }
        Commands::Info => {
            let file_path = &args[2];
            let contents = fs::read(file_path).expect(&format!("Could not read file {file_path}"));
            let decoded_value = decode_bencoded_value(contents);

            println!(
                "Tracker URL: {}",
                decoded_value
                    .get("announce")
                    .unwrap()
                    .to_string()
                    .trim_matches('\"')
            );
            println!(
                "Length: {}",
                decoded_value.get("info").unwrap().get("length").unwrap()
            );
        }
    }
}
