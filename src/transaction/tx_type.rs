use std::fmt;
use serde_bytes::ByteBuf;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use super::asset::{AssetID, AssetParams};
use crate::accounts::{PublicKeyBytes, AddressBytes};
use crate::errors::{Error, AlgorandSdkError};

pub const DIGEST_BYTE_LENGTH: usize = 32;

/// Indentifies the type of the transaction
#[derive(Clone, Copy, Debug)]
pub enum TxType {
  // Type for Payment Transactions
  Payment,
  // Type for Key registrations
  KeyReg,
  // Type for transaction that creates, re-configures or destroys an asset
  AssetConfig
}

impl TxType {
  pub fn from_str(tx_type: &str) -> Result<TxType, Error> {
    match tx_type {
      "pay" => Ok(TxType::Payment),
      "keyreg" => Ok(TxType::KeyReg),
      "acfg" => Ok(TxType::AssetConfig),
      others => Err(AlgorandSdkError::GenericError(format!("Unknown transaction type {}", others)))?,
    }
  }

  pub fn to_str(&self) -> &'static str {
    match self {
      TxType::Payment => "pay",
      TxType::KeyReg => "keyreg",
      TxType::AssetConfig => "acfg",
    }
  }
}

struct TxTypeStringVisitor;

impl<'de> de::Visitor<'de> for TxTypeStringVisitor {
    type Value = TxType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string containing transaction type data")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        TxType::from_str(v).map_err(E::custom)
    }
}

pub fn serialize<S>(t: &TxType, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer 
{
  serializer.serialize_str(t.to_str())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<TxType, D::Error>
where D: Deserializer<'de>
{
  deserializer.deserialize_str(TxTypeStringVisitor)
}

pub type MicroAlgos = u64;
pub type Round = u64;

/// This is for internal use only. Primarily for encoding and sending over the network
#[derive(Debug, Deserialize, Serialize)]
pub struct RawTransaction {
  // NOTE: All fields should be in alphabetical order for encoding to work properly
  #[serde(rename = "amt", skip_serializing_if = "Option::is_none")]
  pub amount: Option<MicroAlgos>,

  #[serde(rename = "apar", skip_serializing_if = "Option::is_none")]
  pub asset_params: Option<AssetParams>,

  #[serde(rename = "caid", skip_serializing_if = "Option::is_none")]
  pub asset_id: Option<AssetID>,

  /// When close_remainder_to is set, it indicates that the
  /// transactio is requesting that the account should be closed, and all remaining
  /// funds be transferred to this address.
  #[serde(rename = "close", skip_serializing_if = "Option::is_none")]
  pub close_remainder_to: Option<ByteBuf>,

  pub fee: MicroAlgos,

  #[serde(rename = "fv")]
  pub first_valid: Round,

  #[serde(rename = "gen", skip_serializing_if = "String::is_empty")]
  pub genesis_id: String,

  #[serde(rename = "gh")]
  pub genesis_hash: ByteBuf,

  /// Group specifies that this transaction is part of a transaction group
  /// And if so specifies the hash of a TxGroup)
  #[serde(rename = "grp", skip_serializing_if = "Option::is_none")]
  pub group: Option<ByteBuf>,

  #[serde(rename = "lv")]
  pub last_valid: Round,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub note: Option<ByteBuf>,

  #[serde(rename = "rcv", skip_serializing_if = "Option::is_none")]
  pub receiver: Option<ByteBuf>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub selkey: Option<ByteBuf>,

  #[serde(rename = "snd")]
  pub sender: ByteBuf,

  #[serde(with = "super::tx_type", rename = "type")]
  pub tx_type: TxType,
  
  #[serde(skip_serializing_if = "Option::is_none")]
  pub votefst: Option<Round>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub votekd: Option<u64>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub votekey: Option<ByteBuf>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub votelst: Option<Round>,
}

#[derive(Clone, Debug)]
pub struct TransactionHeader {
  pub sender: AddressBytes,
  pub fee: MicroAlgos,
  pub first_valid: Round,
  pub last_valid: Round,
  pub note: Option<Vec<u8>>,
  pub genesis_id: String,
  pub genesis_hash: [u8; DIGEST_BYTE_LENGTH],
  pub group: Option<[u8; DIGEST_BYTE_LENGTH]>,
}

/// Payment Transaction Parameters captures fields used by payment transactions
#[derive(Clone, Debug)]
pub struct PaymentTransactionParams {
  pub receiver: AddressBytes,
  pub amount: MicroAlgos,

  /// When close_remainder_to is set, it indicates that the
	/// transaction is requesting that the account should be
	/// closed, and all remaining funds be transferred to this
	/// address.
  pub close_remainder_to: Option<AddressBytes>,
}

/// Captures fields used by key registrations transactions
#[derive(Clone, Debug)]
pub struct KeyRegTransactionParams {
  pub vote_pk: PublicKeyBytes,
  pub selection_pk: PublicKeyBytes,
  pub vote_first: Round,
  pub vote_last: Round,
  pub vote_key_dilution: u64,
}