use failure::Fail;
pub use failure::Error;

#[derive(Debug, Fail)]
pub enum AlgorandSdkError {
  // Mnemonic Errors
  #[fail(display = "Invalid Mnemonic Phrase. Should have 25 words but got: {}", _0)]
  InvalidPhrase(String),
  #[fail(display = "Invalid word [{}] found in phrase", _0)]
  InvalidPhraseWord(String),
  #[fail(display = "Invalid Checksum")]
  InvalidChecksum(),
  #[fail(display = "Invalid Seed for Mnemonic")]
  InvalidSeed(),
  #[fail(display = "Error with mnemonic: {}", _0)]
  GenericError(String),

  // Account Errors
  #[fail(display = "Invalid checksum address {}", _0)]
  InvalidChecksumAddress(String),
  #[fail(display = "Wrong address length, should be {} length got {}", _0, _1)]
  WrongAddressLength(usize, usize),
  #[fail(display = "Wrong address byte length, should be {} length got {}", _0, _1)]
  WrongAddressByteLength(usize, usize),
}