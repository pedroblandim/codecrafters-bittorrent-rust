use std::{env, str::FromStr};

use serde_json::Number;

use crate::types::bencode_types::BencodeTypes;

mod types;

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    let Ok(decoded) = BencodeTypes::from_str(encoded_value) else {
        panic!("Unhandled encoded value: {}", encoded_value);
    };

    match decoded {
        BencodeTypes::ByteString(string) => serde_json::Value::String(string.to_string()),
        BencodeTypes::Integer(number) => serde_json::Value::Number(Number::from(number)),
    }
}

// Usage: your_program.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // You can use print statements as follows for debugging, they'll be visible when running tests.
        eprintln!("Logs from your program will appear here!");

        // TODO: Uncomment the code below to pass the first stage
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
