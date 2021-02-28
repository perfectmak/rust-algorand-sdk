mod asset;
mod tx_type;
mod inputs;

use std::convert::TryInto;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use sha2::{Digest, Sha512Trunc256};
use asset::{AssetConfigTransactionParams};
use tx_type::{RawTransaction, TransactionHeader, PaymentTransactionParams, KeyRegTransactionParams};
use crate::accounts::{Account, Signature};
use crate::errors::{Error};
use crate::encoding::{rmp_encode, base32_encode};

pub use inputs::{AssetConfigTransactionInput, PaymentTransactionInput, KeyRegTransactionInput, TransactionInput};
pub use tx_type::{TxType, MicroAlgos, Round};

const MINIMUM_TX_FEE: u64 = 1000;

/// Transaction describes a transaction that can appear in a block.
#[derive(Clone, Debug)]
pub struct Transaction {
  pub tx_type: TxType,
  pub header: TransactionHeader,
  pub payment_params: Option<PaymentTransactionParams>,
  pub key_reg_params: Option<KeyRegTransactionParams>,
  pub asset_config_params: Option<AssetConfigTransactionParams>,
}

impl Transaction {
  /// Constructs a new transaction based on the type of the input.
  pub fn from_input<T: TransactionInput>(input: T) -> Result<Transaction, Error> {
    let (tx_type, header) = input.build_header()?;
    let txn = Transaction {
      tx_type,
      header,
      payment_params: input.build_payment_params()?,
      key_reg_params: input.build_key_reg_params()?,
      asset_config_params: input.build_asset_config_params()?,
    };

    Ok(input.modify_final_transaction(txn)?)
  }

  // Signs this transaction with the account/private key provided
  pub fn sign(&self, account: &Account) -> Result<SignedTransaction, Error> {
    let bytes_to_sign = self.to_raw_bytes()?;
    let signature = account.sign(bytes_to_sign.as_ref());

    // compute id from signature
    let checksum = Sha512Trunc256::default().chain(bytes_to_sign).result();
    let txn_id = base32_encode(checksum.as_ref());

    Ok(SignedTransaction {
      txn_id,
      signature,
      transaction: self.to_raw(),
      multisig_sig: None,
    })
  }

  // Get raw bytes from encoding this transaction
  // The returned byte can be signed for a signed transaction
  pub fn to_raw_bytes(&self) -> Result<Vec<u8>, Error> {
    let raw_txn = self.to_raw();
    Ok(Transaction::with_encode_tag(&rmp_encode(&raw_txn)?))
  }

  fn with_encode_tag(bytes: &Vec<u8>) -> Vec<u8> {
    let mut tag: Vec<u8> = Vec::new();
    tag.extend(b"TX");
    tag.extend(bytes);
    tag
  }

  /// Estimates the byte size of transaction when sent
  fn estimate_size(&self) -> Result<u64, Error> {
    let account = Account::generate();
    let signed_txn = self.sign(&account)?;

    let signed_txn_bytes = signed_txn.encode()?;
    Ok(signed_txn_bytes.len().try_into()?)
  }

  // Convert `Transaction` to `RawTransaction` that is encodable
  fn to_raw(&self) -> RawTransaction {
    let raw_txn = RawTransaction {
      tx_type: self.tx_type,
      // headers
      sender: ByteBuf::from(self.header.sender.to_vec()),
      fee: self.header.fee,
      first_valid: self.header.first_valid,
      last_valid: self.header.last_valid,
      note: self.header.note.as_ref().map(|n| ByteBuf::from(n.clone())),
      genesis_id: self.header.genesis_id.clone(),
      genesis_hash: ByteBuf::from(self.header.genesis_hash.to_vec()),
      group: self.header.group.map(|g| ByteBuf::from(g.to_vec())),

      // key reg fields
      votekey: self.key_reg_params.as_ref().map(|k| ByteBuf::from(k.vote_pk.to_vec())),
      selkey: self.key_reg_params.as_ref().map(|k| ByteBuf::from(k.selection_pk.to_vec())),
      votefst: self.key_reg_params.as_ref().map(|k| k.vote_first),
      votelst: self.key_reg_params.as_ref().map(|k| k.vote_last),
      votekd: self.key_reg_params.as_ref().map(|k| k.vote_key_dilution),

      // payment fields
      receiver: self.payment_params.as_ref().map(|p| ByteBuf::from(p.receiver.to_vec())),
      amount: self.payment_params.as_ref().map(|p| p.amount),
      // extract inner remainder too. would be easier if flatten() was stable
      close_remainder_to: match self.payment_params.as_ref() {
        Some(p) => match p.close_remainder_to {
          Some(remainder_addr) => Some(ByteBuf::from(remainder_addr.to_vec())),
          _ => None,
        },
        _ => None,
      },

      // asset txn fields
      asset_id: self.asset_config_params.as_ref().map(|a| a.asset_id.clone()),
      asset_params: match self.asset_config_params.as_ref() {
        Some(a) => match a.asset_params.as_ref() {
          Some(asset_params) => Some(asset_params.clone()),
          _ => None,
        },
        _ => None,
      },
    };

    raw_txn
  }
}

#[derive(Serialize, Deserialize)]
pub struct MultisigSubsig {
  #[serde(rename = "pk")]
  pub key: ByteBuf, // ed25519 public key

  #[serde(rename = "s")]
  pub signature: ByteBuf,
}

/// MultisigSig holds multiple Subsigs, as well as threshold and version info
#[derive(Serialize, Deserialize)]
pub struct MultisigSig {
  #[serde(rename = "subsig")]
  pub subsigs: Vec<MultisigSubsig>,

  #[serde(rename = "thr")]
  pub threshold: u8,

  #[serde(rename = "v")]
  pub version: u8,
}

/// SignedTransaction wraps a transaction and a signature. The rmp encoding of this 
/// struct is suitable to broadcast on the network
#[derive(Serialize, Deserialize)]
pub struct SignedTransaction {
  #[serde(rename = "msig", skip_serializing_if = "Option::is_none")]
  pub multisig_sig: Option<ByteBuf>,

  #[serde(rename = "sig")]
  pub signature: Signature,

  #[serde(rename = "txn")]
  transaction: RawTransaction,

  #[serde(skip)]
  pub txn_id: String,
}

impl SignedTransaction {
  pub fn encode(&self) -> Result<Vec<u8>, Error> {
    Ok(rmp_encode(self)?)
  }
}

#[cfg(test)]
mod tests;