use std::fs;
use std::{env, str::FromStr};

use sha1::digest::consts::{B0, B1};
use sha1::digest::generic_array::GenericArray;
use sha1::digest::typenum::{UInt, UTerm};
use sha1::{Digest, Sha1};

use crate::client::make_request;
use crate::{bencode::bencode_types::BencodeTypes, commands::Commands};

mod bencode;
mod client;
mod commands;

// Usage: your_program.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command_string = match args.get(1) {
        Some(s) => s,
        None => "peers",
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

            let announce_bytes = content_decoded.get_byte_string("announce").unwrap();
            let announce = String::from_utf8_lossy(&announce_bytes);

            let info = content_decoded.get_dict("info").unwrap();
            let length = info.get_integer("length").unwrap();

            let info_encoded = BencodeTypes::encode(info);
            let info_encoded_hash = Sha1::digest(info_encoded.clone());

            let piece_length = info.get_integer("piece length").unwrap();

            let pieces_string = info.get_byte_string("pieces").unwrap();

            let pieces = pieces_string.chunks(20);

            println!("Tracker URL: {}", announce);
            println!("Length: {}", length);
            println!("Info Hash: {info_encoded_hash:x}");
            println!("Piece Length: {piece_length}");

            println!("Piece Hashes:");

            pieces.for_each(|p| {
                let ga: GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>> =
                    GenericArray::clone_from_slice(p);
                println!("{:x}", ga);
            });
        }
        Commands::Peers => {
            let file_path = match args.get(2) {
                Some(p) => p,
                None => "sample.torrent",
            };

            let content = fs::read(file_path).expect(&format!("Could not read file {file_path}"));

            let content_decoded = BencodeTypes::decode(content.clone());

            let announce_bytes = content_decoded.get_byte_string("announce").unwrap();
            let announce = String::from_utf8_lossy(&announce_bytes);

            let info = content_decoded.get_dict("info").unwrap();

            let info_encoded = BencodeTypes::encode(info);
            let info_encoded_hash = Sha1::digest(info_encoded.clone());

            let piece_length = info.get_integer("piece length").unwrap();

            let res = make_request(&announce.to_string(), info_encoded_hash, piece_length);
            match res.get_byte_string("failure reason") {
                Some(error_reason) => panic!(
                    "API returned error: {}",
                    String::from_utf8_lossy(&error_reason)
                ),
                None => (),
            };

            let Some(peers_string) = res.get_byte_string("peers") else {
                return ();
            };

            const PEERS_SIZE: u8 = 6;
            const IP_PART_SIZE: u8 = 4;

            let peers = peers_string.chunks(PEERS_SIZE as usize);

            peers.for_each(|p| {
                let parts = p.to_vec();

                let mut chunks = parts.chunks(IP_PART_SIZE as usize);

                let ips = chunks
                    .next()
                    .unwrap()
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(".");

                let port_bytes = chunks.next().unwrap();
                let port = (u32::from(port_bytes[0]) << 8) + port_bytes[1] as u32;

                println!("{ips}:{port}");
            });
        }
    }
}
