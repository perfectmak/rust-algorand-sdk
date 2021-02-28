use serde::{de::DeserializeOwned, Serialize};
use rmp_serde::{encode, decode, from_read};
use base32::{Alphabet, encode as base32_enc, decode as base32_dec};
pub use base64::{decode as base64_decode, encode as base64_encode};

/// rmp encodes the serialized data.
/// NOTE: Any struct to be serialized should have its fields sorted
/// as per required by algorand blockchain
pub fn rmp_encode<Data: Serialize>(data: &Data) -> Result<Vec<u8>, encode::Error> {
  encode::to_vec_named(data)
}

/// rmp decodes the byte array reference into type result.
/// Type must implement DeserializedOwned
pub fn rmp_decode<Data: DeserializeOwned>(buffer: &[u8]) -> Result<Data, decode::Error> {
  from_read(buffer)
}

pub fn base32_encode(data: &[u8]) -> String {
  base32_enc(Alphabet::RFC4648 { padding: false }, data)
}

pub fn base32_decode(data: &str) -> Option<Vec<u8>> {
  base32_dec(Alphabet::RFC4648 { padding: false }, data)
}

#[cfg(test)]
mod tests {
  use serde::{Deserialize, Serialize};
  use super::{rmp_encode, rmp_decode};

  #[test]
  fn encode_decode_works() {
    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct Person {
      name: String,
      age: u16,
    }
    let expected_value = Person {
      name: "Perfect".into(),
      age: 25,
    };

    let bytes = rmp_encode(&expected_value).unwrap();

    let actual_value: Person = rmp_decode(&bytes).unwrap();
    assert_eq!(actual_value, expected_value);
  }

  #[test]
  fn encode_works_with_known_bytes() {
    #[derive(Deserialize, Serialize)]
    struct Obj {
      #[serde(rename = "A")]
      a: u32
    }
    let expected_bytes_1: Vec<u8> = vec![0x81, 0xa1, 0x41, 0x78];
    let expected_bytes_2: Vec<u8> = vec![0x81, 0xa1, 0x41, 0xcd, 0x1, 0x2c];

    let actual_bytes_1 = rmp_encode(&Obj {
      a: 120
    }).unwrap();

    let actual_bytes_2 = rmp_encode(&Obj {
      a: 300
    }).unwrap();

    assert_eq!(actual_bytes_1, expected_bytes_1);
    assert_eq!(actual_bytes_2, expected_bytes_2);
  }

  #[test]
  fn should_encode_binary_blob_and_string_properly() {
    #[derive(Deserialize, Serialize)]
    struct Obj {
      #[serde(with = "serde_bytes", rename = "J")]
      j: Vec<u8>,
      #[serde(rename = "K")]
      k: String,
    }

    let expected_bytes: Vec<u8> = vec![0x82, 0xa1, 0x4a, 0xc4, 0x3, 0x14, 0x1e, 0x28, 0xa1, 0x4b, 0xa3, 0x61, 0x61, 0x61];

    let actual_bytes = rmp_encode(&Obj {
      j: vec![20, 30, 40],
      k: "aaa".into(),
    }).unwrap();

    assert_eq!(actual_bytes, expected_bytes);
  }

  #[test]
  fn should_match_go_code() {
    use serde_bytes::ByteBuf;
    use crate::accounts::Address;

    #[derive(Deserialize, Serialize)]
    struct Obj {
      amt: u16,
      fee: u16,
      fv: u16,
      lv: u16,
      #[serde(with = "serde_bytes")]
      rcv: Vec<u8>,
      snd: ByteBuf,
    }

    let expected_bytes: Vec<u8> = vec![134, 163, 97, 109, 116, 205, 3, 79, 163, 102, 101, 101, 10, 162, 102, 118, 51, 162, 108, 118, 61, 163, 114, 99, 118, 196, 32, 145, 154, 160, 178, 192, 112, 147, 3, 73, 200, 52, 23, 24, 49, 180, 79, 91, 78, 35, 190, 125, 207, 231, 37, 41, 131, 96, 252, 244, 221, 54, 208, 163, 115, 110, 100, 196, 32, 145, 154, 160, 178, 192, 112, 147, 3, 73, 200, 52, 23, 24, 49, 180, 79, 91, 78, 35, 190, 125, 207, 231, 37, 41, 131, 96, 252, 244, 221, 54, 208];
    let address_str = "SGNKBMWAOCJQGSOIGQLRQMNUJ5NU4I56PXH6OJJJQNQPZ5G5G3IOVLI5VM";
    let o = Obj {
      snd: ByteBuf::from(Address::from_string(address_str).unwrap().to_vec()),
      rcv: Address::from_string(address_str).unwrap().to_vec(),
      fee: 10,
      amt: 847,
      fv: 51,
      lv: 61,
    };

    let actual_bytes = rmp_encode(&o).unwrap();

    assert_eq!(actual_bytes, expected_bytes);
  }
}