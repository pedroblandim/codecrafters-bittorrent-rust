use std::io::{self, Write, stdin};
use std::process::ExitCode;
use std::{env, str::FromStr};

use crate::{
    commands::Commands,
    types::bencode_types::{BencodeTypes, ToJSON},
};

mod commands;
mod types;

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    let decoded = BencodeTypes::parse(encoded_value);

    decoded.serialize()
}

// Usage: your_program.sh decode "<encoded_value>"
fn main() {
    loop {
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        let stdin = stdin();
        stdin
            .read_line(&mut buffer)
            .ok()
            .expect("Failed to read line");

        let input = buffer.trim();
        let parts = input.split(" ").collect::<Vec<_>>();
        // let command_string = &parts[0];
        let encoded_string = &parts[0];

        let Ok(command) = Commands::from_str("decode") else {
            println!("unknown command: {}", "decode");
            return ();
        };

        match command {
            Commands::Decode => {
                let encoded_value = encoded_string;
                let decoded_value = decode_bencoded_value(encoded_value);
                println!("{}", decoded_value.to_string());
            }
        }
    }
}
