use std::fs;
use std::{env, str::FromStr};

use crate::{bencode::bencode_types::BencodeTypes, commands::Commands};

mod bencode;
mod commands;

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

            let decoded_value = BencodeTypes::decode(encoded_value.to_owned().into_bytes());

            let json_value = decoded_value.serialize_to_json();

            println!("{}", json_value.to_string());
        }
        Commands::Info => {
            let file_path = &args[2];
            let contents = fs::read(file_path).expect(&format!("Could not read file {file_path}"));

            let decoded_value = BencodeTypes::decode(contents.clone());

            let decoded_json = decoded_value.serialize_to_json();

            let info = decoded_json.get("info").unwrap();

            let deserialized_info = BencodeTypes::deserialize_from_json(info).unwrap();

            let encoded_info = BencodeTypes::encode(&deserialized_info);

            println!(
                "Tracker URL: {}",
                decoded_json
                    .get("announce")
                    .unwrap()
                    .to_string()
                    .trim_matches('\"')
            );
            println!(
                "Length: {}",
                decoded_json.get("info").unwrap().get("length").unwrap()
            );

            println!("Info: {:?}", decoded_json.get("info").unwrap());
            println!("Encoded Info: {:?}", str::from_utf8(&encoded_info).unwrap());

            let d = BencodeTypes::decode(contents.clone());
            let e = BencodeTypes::encode(&d);
            println!("\n\n");
            println!("{:?}\n", String::from_utf8_lossy(&e));
            println!("{:?}", String::from_utf8_lossy(&contents));
        }
    }
}
