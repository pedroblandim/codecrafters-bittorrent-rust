use crate::bencode::bencode_types::BencodeTypes;

impl BencodeTypes {
    pub fn encode(decoded_value: &Self) -> Vec<u8> {
        match decoded_value {
            BencodeTypes::ByteString(string) => {
                let mut bytes: Vec<u8> = string.len().to_string().into_bytes();

                bytes.push(':' as u8);
                bytes.append(&mut string.clone());

                bytes
            }
            BencodeTypes::Integer(number) => {
                let mut bytes: Vec<u8> = vec!['i' as u8];

                let mut number_bytes = number.to_string().clone().into_bytes();
                bytes.append(&mut number_bytes);

                bytes.push('e' as u8);

                bytes
            }
            BencodeTypes::List(list) => {
                let mut bytes = vec!['l' as u8];

                for element in list {
                    bytes.append(&mut BencodeTypes::encode(element));
                }

                bytes.push('e' as u8);

                bytes
            }
            BencodeTypes::Dictionary(dict) => {
                let mut bytes = vec!['d' as u8];

                // keys of bencoded dictonaries must always be sorted
                let mut sorted_keys: Vec<&Vec<u8>> = dict.iter().map(|(k, _)| k).collect();

                sorted_keys.sort();

                for k in sorted_keys {
                    let v = dict.get(k).unwrap();

                    bytes.append(&mut BencodeTypes::encode(&BencodeTypes::ByteString(
                        (*k).clone(),
                    )));
                    bytes.append(&mut BencodeTypes::encode(v));
                }

                bytes.push('e' as u8);

                bytes
            }
        }
    }
}
