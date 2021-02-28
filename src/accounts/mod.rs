pub mod mnemonics;
mod address;

use rand::rngs::OsRng;
use ed25519_dalek::{PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH};
use ed25519_dalek::ExpandedSecretKey;

pub use ed25519_dalek::{PublicKey, SecretKey, Signature};
pub use address::{Address, AddressBytes};

use mnemonics::seed_from_mnemonic;
use crate::errors::{Error};

pub type PublicKeyBytes = [u8; PUBLIC_KEY_LENGTH];
pub type SecretKeyBytes = [u8; SECRET_KEY_LENGTH];

/// An account used for signing transactions.
/// 
/// An `Account` represents the fully formed account containing
/// both the public key/address and private key.
/// You can only use an Account instance to sign messages.
/// 
/// If you are interested in only a public key/address representation of a user
/// to verify a signed transaction then you should use the `Address` object instead
/// 
pub struct Account {
  private_key: SecretKey,
  public_key: PublicKey,
  pub address: Address,
}

impl Account {
  /// This generate a random Account.
  /// If you have seed bytes from an existing account, you should use the
  /// [`Account::from_key()`] function instead, or if you have a mnemonic phrase
  /// then use the [`Account::from_mnemonic()`] function to create an account.
  /// 
  /// # Example
  /// ```rust
  /// use rust_algorand_sdk::accounts::Account;
  /// 
  /// let account = Account::generate();
  /// let signature = account.sign(&[1, 2, 3]);
  /// // signature represents the signed bytes
  /// println!("Signature {:?}", signature)
  /// ```
  pub fn generate() -> Account {
    let mut csprng: OsRng = OsRng::new().unwrap();
    let private_key = SecretKey::generate(&mut csprng); 
    let public_key: PublicKey = (&private_key).into();
    Account {
      address: Address::from_fixed_bytes(public_key.to_bytes()),
      private_key,
      public_key,
    }
  }

  /// Create an account from a known mnemonic phrase
  /// 
  /// Note: The phrase words count must be 25. 24 words used to generate
  /// the key and the last word should be the checksum word.
  /// 
  /// # Returns
  /// A `Result` with okay value being an `Account` or with error being 
  /// a `failure::Error` wrapping the internal error that occurred.
  pub fn from_mnemonic(mnemonic: &str) -> Result<Account, Error> {
    let seed = seed_from_mnemonic(mnemonic)?;
    Account::from_key(seed.as_ref())
  }

  /// Create an account from a set of know seed key bytes.
  /// 
  /// Note: the bytes must be of size [`SEED_BYTES_LENGTH`] which is 32
  pub fn from_key(bytes: &[u8]) -> Result<Account, Error> {
    let private_key = SecretKey::from_bytes(bytes)?;
    let public_key: PublicKey = (&private_key).into();
    
    Ok(Account {
      address: Address::from_fixed_bytes(public_key.to_bytes()),
      private_key,
      public_key,
    })
  }

  pub fn sign(&self, message: &[u8]) -> Signature {
    let expanded: ExpandedSecretKey = (&self.private_key).into();
    expanded.sign(&message, &self.public_key)
  }
}

/// A type for representing multisig preimage data
pub struct MultisigAccount {
  version: u8,
  threshold: u8,
  public_keys: Vec<PublicKey>, 
}

impl MultisigAccount {
  
}

#[cfg(test)]
mod tests {
  use super::{Account};

  #[test]
  fn test_account_generation() {
    let account = Account::generate();
    // address should be equal to public key
    assert_eq!(account.address.as_bytes(), account.public_key.to_bytes());
  }
}