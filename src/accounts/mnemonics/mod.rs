//! Mnemonic and Seed implementation logic.
//! 
//! The Mnenmonic is usually used to generate a Seed for creating an Account's Keypair

use sha2::{Digest, Sha512Trunc256};
use wordlist::WORDLIST;
use crate::errors::{AlgorandSdkError, Error};

mod wordlist;

pub const MNEMONIC_PHRASE_WORD_COUNT: usize = 24;
pub const SEED_BYTES_LENGTH: usize = 32; 



/// Generate the seed/key from mnemonic phrase
/// 
pub fn seed_from_mnemonic(phrase: &str) -> Result<Vec<u8>, Error> {
  let words: Vec<&str> = phrase.split(' ')
    .collect();
  // NOTE: word_len excludes the checksum from the list
  let word_len = words.len() - 1;

  // validate phrase length
  if word_len != MNEMONIC_PHRASE_WORD_COUNT {
    return Err(AlgorandSdkError::InvalidPhrase(String::from(phrase)))?;
  }

  let checksum = words.last().unwrap();
  let mut u11_seed: Vec<u32> = Vec::with_capacity(word_len);

  // validate phrase words
  for word in &words[..word_len] {
    if let Ok(idx) = WORDLIST.binary_search(word) {
        u11_seed.push(idx as u32);
    } else {
        return Err(AlgorandSdkError::InvalidPhraseWord(word.to_string()))?;
    }
  }

  let entropy = to_byte_array(u11_seed.as_ref());

  // validate we have 33 bytes (i.e (24 words indices) * 11 bits per word) works)
  if entropy.len() != 33 {
    return Err(AlgorandSdkError::GenericError("Failed to decode mnemonic".into()))?;
  }

  // also ensure last bytes is 0x00
  if *entropy.last().unwrap() != (0x00 as u8) {
    return Err(AlgorandSdkError::GenericError("Failed to decode mnemonic bytes".into()))?;
  }

  // chop off 0 bytes to get seed entropy
  let seed = &entropy[..(entropy.len() - 1)];
  
  // validate checksum
  let computed_checksum = compute_checksum(seed);
  if computed_checksum != *checksum {
    return Err(AlgorandSdkError::InvalidChecksum())?;
  }

  Ok(Vec::from(seed))
}

pub fn mnemonic_from_seed(seed: &[u8]) -> Result<String, Error> {
  if seed.len() != SEED_BYTES_LENGTH {
    return Err(AlgorandSdkError::InvalidSeed())?;
  }

  let u11_seed = to_u11_array(seed);
  let words = apply_words(u11_seed.as_ref()).join(" ");

  let checksum = compute_checksum(seed);
  let result = format!("{} {}", words, checksum);

  Ok(result)
}

/// Returns a word from WORDLIST that is the checksum workd
fn compute_checksum(seed: &[u8]) -> String {
  let hasher = Sha512Trunc256::default();
  let output = hasher.chain(seed).result();

  let u11_array = to_u11_array(&output[..2]);
  apply_words(u11_array.as_ref())
    .first()
    .unwrap()
    .clone()
}

fn apply_words(arr: &[u32]) -> Vec<String> {
  arr.iter()
    .map(|idx| String::from(WORDLIST[*idx as usize]))
    .collect()
}

fn to_u11_array(arr: &[u8]) -> Vec<u32> {
	let mut buffer: u32 = 0;
	let mut number_of_bit: u16 = 0;
	let mut output: Vec<u32> = Vec::new();

	for value in arr {
		// prepend bits to buffer
		buffer |= (*value as u32) << number_of_bit;
		number_of_bit += 8;

		// if there enough bits, extract 11bit number
		if number_of_bit >= 11 {
			// 0x7FF is 2047, the max 11 bit number
			output.push(buffer & 0x7FF);

			// drop chunk from buffer
			buffer = buffer >> 11;
			number_of_bit -= 11;
		}
	}

	if number_of_bit != 0 {
		output.push(buffer & 0x7FF);
	}

	output
}

/// Serialize the 11 bits arr to 8 bit
/// NOTE: We are using 32 bits input instead of 16 bits as input to accomodate for the shifting
/// which sometimes moves out of 16 bits
fn to_byte_array(arr: &[u32]) -> Vec<u8> {
	let mut buffer: u32 = 0x0000;
	let mut number_of_bit: u8 = 0;
	let mut output: Vec<u8> = Vec::with_capacity(11 * arr.len() / 8);

	for value in arr {
		buffer = ((*value << number_of_bit) | buffer).into();
		number_of_bit += 11;

		while number_of_bit >= 8 {
			output.push((buffer & 0xff) as u8);
			buffer >>= 8;
			number_of_bit -= 8;
		}
	}

	if number_of_bit != 0 {
		output.push((buffer & 0xff) as u8);
	}

	output
}

#[cfg(test)]
mod tests {
  use super::{mnemonic_from_seed, seed_from_mnemonic, AlgorandSdkError};

  #[test]
  fn mnemonic_from_seed_should_pass_for_zero_vector() {
    let seed = &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mnemonic = mnemonic_from_seed(seed).unwrap();

    let expected_phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon invest";

    assert_eq!(mnemonic, expected_phrase);
  }

  // #[should_not_panic]
  #[test]
  fn seed_from_mnemonic_back_to_seed_works() {
    use rand::Rng;

    for _i in 0..100 {
      let seed = rand::thread_rng().gen::<[u8; 32]>();
      let mnemonic = mnemonic_from_seed(&seed).unwrap();
      seed_from_mnemonic(&mnemonic).unwrap();
    }

    assert!(true)
  }

  #[test]
  fn seed_from_mnemonic_should_fail_with_wrong_checksum() {
    use rand::Rng;
    let seed = rand::thread_rng().gen::<[u8; 32]>();
    let mut mnemonic = mnemonic_from_seed(&seed).unwrap();
    
    // modify checksum (last word)
    mnemonic.push('n');

    let actual_error = seed_from_mnemonic(&mnemonic).unwrap_err();
    if let AlgorandSdkError::InvalidChecksum() = actual_error.downcast_ref().unwrap() { /* everything is okay */ }
    else {
      assert!(false, "Not invalid phrase word")
    }

    assert!(true)
  }

  #[test]
  fn seed_from_mnemonic_should_fail_if_mnemonic_is_invalid() {
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon venue abandon abandon abandon abandon abandon abandon abandon abandon abandon invest";

    let actual_error = seed_from_mnemonic(&mnemonic).unwrap_err();
    if let AlgorandSdkError::InvalidChecksum() = actual_error.downcast_ref().unwrap() { /* everything is okay */ }
    else {
      assert!(false, "Not invalid phrase word")
    }
  }

  #[test]
  fn seed_from_mnemonic_should_fail_if_contains_invalid_word() {
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon venues abandon abandon abandon abandon abandon abandon abandon abandon abandon invest";

    let actual_error = seed_from_mnemonic(&mnemonic).unwrap_err();
    if let AlgorandSdkError::InvalidPhraseWord(_) = actual_error.downcast_ref().unwrap() {}
    else {
      assert!(false, "Not invalid phrase word")
    }
  }
}