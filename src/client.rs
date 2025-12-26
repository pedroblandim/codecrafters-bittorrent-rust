use regex::bytes::Regex;
use reqwest::{Url, blocking};
use sha1::digest::{
    consts::{B0, B1},
    generic_array::GenericArray,
    typenum::{UInt, UTerm},
};
use std::io::Read;

use crate::bencode::bencode_types::BencodeTypes;

const PEER_ID: &str = "235d860da2a4484c8908";

pub fn make_request(
    tracker_url: &str,
    info_hash: GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>>,
    length: i64,
) -> BencodeTypes {
    let mut info_hash_string = String::new();

    // encode info_hash
    let re = Regex::new(r"[0-9a-zA-Z\.\-_~]").unwrap();
    for b in info_hash.bytes() {
        let b = b.unwrap();

        if re.is_match(&[b]) {
            info_hash_string.push(b as char);
        } else if b as char == ' ' {
            info_hash_string.push_str("+");
        } else {
            info_hash_string.push_str("%");
            info_hash_string.push_str(&hex::encode(vec![b]).to_string().to_uppercase());
        }
    }

    let url = Url::parse_with_params(
        &(tracker_url.to_owned() + format!("?info_hash={info_hash_string}").as_str()),
        &[
            ("peer_id", PEER_ID.to_string()),
            ("port", "6881".to_string()),
            ("downloaded", "0".to_string()),
            ("uploaded", "0".to_string()),
            ("left", length.to_string()),
            ("compact", "1".to_string()),
        ],
    )
    .unwrap();

    let mut res = blocking::get(url).unwrap();

    let mut body = vec![];
    res.read_to_end(&mut body).unwrap();

    BencodeTypes::decode(body)
}
