use sha2::{Digest, Sha512Trunc256};
use crate::helpers::ToArray;
use crate::encoding::{base32_decode, base32_encode};
use crate::errors::{AlgorandSdkError, Error};

pub const CHECKSUM_BYTES_LENGTH: usize = 4;
pub const ADDRESS_BYTES_LENGTH: usize = 32;

pub type AddressBytes = [u8; ADDRESS_BYTES_LENGTH];

#[derive(Debug)]
pub struct Address(AddressBytes);

impl Address {
  /// Creates an Address from the checksum address string
  /// 
  /// # Examples
  /// 
  /// ```
  /// use rust_algorand_sdk::accounts::Address;
  /// 
  /// let address = Address::from_string("47YPQTIGQEO7T4Y4RWDYWEKV6RTR2UNBQXBABEEGM72ESWDQNCQ52OPASU").unwrap();
  /// 
  /// 
  /// ```
  pub fn from_string(address_str: &str) -> Result<Address, Error> {
    let optional_address_with_checksum = base32_decode(address_str);
    if let None = optional_address_with_checksum {
      return Err(AlgorandSdkError::InvalidChecksumAddress(String::from(address_str)))?;
    }

    let address_with_checksum = optional_address_with_checksum.unwrap();
    let address_length = address_with_checksum.len();
    let expected_address_length = CHECKSUM_BYTES_LENGTH + ADDRESS_BYTES_LENGTH;
    if address_length != expected_address_length {
      return Err(AlgorandSdkError::WrongAddressLength(expected_address_length, address_length))?;
    }

    let address_bytes = &address_with_checksum[..ADDRESS_BYTES_LENGTH];
    let checksum_bytes = &address_with_checksum[ADDRESS_BYTES_LENGTH..];

    // compute and compare checksum
    let checksum = Sha512Trunc256::default().chain(address_bytes).result();
    let expected_checksum_bytes = &checksum[ADDRESS_BYTES_LENGTH - CHECKSUM_BYTES_LENGTH..];

    if expected_checksum_bytes != checksum_bytes {
      return Err(AlgorandSdkError::InvalidChecksumAddress(String::from(address_str)))?;
    }

    Ok(Address(address_bytes.to_array()))
  }

  /// Create an `Address` instance from a byte reference. It is a convenience alternative to `from_fixed_bytes` 
  /// 
  /// It will return an Error if the length of the slice is not `ADDRESS_BYTES_LENGTH` (32)
  pub fn from_bytes(address_bytes: &[u8]) -> Result<Address, Error> {
    if address_bytes.len() != ADDRESS_BYTES_LENGTH {
      Err(AlgorandSdkError::WrongAddressByteLength(ADDRESS_BYTES_LENGTH, address_bytes.len()))?;
    }

    let mut result_bytes: [u8; ADDRESS_BYTES_LENGTH] = Default::default();
    result_bytes.copy_from_slice(address_bytes);
    Ok(Address(result_bytes))
  }

  /// Create an `Address` instance from a valid `ADDRESS_BYTES_LENGTH`(32) byte array
  pub fn from_fixed_bytes(address_bytes: [u8; ADDRESS_BYTES_LENGTH]) -> Address {
    Address(address_bytes)
  }

  /// Get checksum string representation of Address
  /// 
  /// # Returns
  /// A human-readable representation of the address. This representation
  /// includes a 4-byte checksum
  pub fn to_string(&self) -> String {
    // compute checksum
    let checksum = Sha512Trunc256::default().chain(self.0).result();
    let checksum_len_bytes = &checksum[ADDRESS_BYTES_LENGTH - CHECKSUM_BYTES_LENGTH..];

    // append checsum to address bytes
    let mut address_with_checksum = Vec::from(&self.0[..]);
    address_with_checksum.extend(checksum_len_bytes);

    base32_encode(address_with_checksum.as_ref())
  }

  pub fn as_bytes(&self) -> &[u8] {
    &self.0
  }

  pub fn into_bytes(self) -> AddressBytes {
    self.0
  }

  pub fn to_vec(&self) -> Vec<u8> {
    self.as_bytes().to_vec()
  }
}

impl Into<AddressBytes> for Address {
  fn into(self) -> AddressBytes {
    self.into_bytes()
  }
}

#[cfg(test)]
mod tests {
  use rand::RngCore;
  use rand::rngs::OsRng;
  use super::{Address, ADDRESS_BYTES_LENGTH};
  

fn random_bytes(csprng: &mut OsRng) -> [u8; ADDRESS_BYTES_LENGTH] {
  let mut filled_bytes: [u8; ADDRESS_BYTES_LENGTH] = Default::default();
  csprng.fill_bytes(&mut filled_bytes);
  filled_bytes
}

  #[test]
  fn test_encode_decode() {
    let mut csprng: OsRng = OsRng::new().unwrap();
    for _ in 0..1000 {
      let address = Address(random_bytes(&mut csprng));
      let address_str = address.to_string();
      // should not panic
      Address::from_string(&address_str).unwrap();
    }
  }

  #[test]
  fn golden_value_encodes() {
    let expected_value = "7777777777777777777777777777777777777777777777777774MSJUVU";
    
    let mut filled_bytes: [u8; ADDRESS_BYTES_LENGTH] = Default::default();
    for i in 0..ADDRESS_BYTES_LENGTH {
      filled_bytes[i] = 0xFF;
    }
    let address = Address(filled_bytes);

    assert_eq!(address.to_string(), expected_value);
  }
}