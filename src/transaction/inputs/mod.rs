use super::{MINIMUM_TX_FEE, TxType, Transaction, TransactionHeader, PaymentTransactionParams, KeyRegTransactionParams, AssetConfigTransactionParams, MicroAlgos, Round};
use serde_bytes::ByteBuf;
use super::tx_type::DIGEST_BYTE_LENGTH;
use super::asset::{AssetID, AssetParams};
use crate::accounts::{Account, Signature, Address, PublicKeyBytes};
use crate::errors::{Error, AlgorandSdkError};
use crate::encoding::{base64_decode};
use crate::helpers::ToArray;

pub trait TransactionInput {
  fn build_header(&self) -> Result<(TxType, TransactionHeader), Error>;

  fn build_payment_params(&self) -> Result<Option<PaymentTransactionParams>, Error> {
    Ok(None)
  }

  fn build_key_reg_params(&self) -> Result<Option<KeyRegTransactionParams>, Error> {
    Ok(None)
  }

  fn build_asset_config_params(&self) -> Result<Option<AssetConfigTransactionParams>, Error> {
    Ok(None)
  }

  fn modify_final_transaction(&self, transaction: Transaction) -> Result<Transaction, Error> {
    Ok(transaction)
  }
}

// default implementation for the build_header and modify_final_transactions
macro_rules! build_header_impl {
  ($type:expr) => {
    fn build_header(&self) -> Result<(TxType, TransactionHeader), Error> {
      let genesis_hash = base64_decode(&self.genesis_hash)?;
      if genesis_hash.is_empty() {
        return Err(AlgorandSdkError::GenericError("Genesis hash required".into()))?;
      }

      if genesis_hash.len() != DIGEST_BYTE_LENGTH {
        return Err(AlgorandSdkError::GenericError(format!(
          "Expected genesis hash to be {} bytes but got {}",
          DIGEST_BYTE_LENGTH,
          genesis_hash.len())),
        )?;
      }

      let header = TransactionHeader {
        sender: Address::from_string(&self.from)?.into(),
        fee: self.fee,
        first_valid: self.first_round,
        last_valid: self.last_round,
        note: self.note.clone(),
        genesis_id: self.genesis_id.clone(),
        genesis_hash: genesis_hash.to_array(),
        group: None,
      };

      

      Ok(($type, header))
    }

    fn modify_final_transaction(&self, transaction: Transaction) -> Result<Transaction, Error> {
      let mut txn = transaction;

      if self.is_flat_fee {
        txn.header.fee = self.fee;
      } else {
        let estimated_size = txn.estimate_size()?;
        txn.header.fee = estimated_size * self.fee;
      }

      if txn.header.fee < MINIMUM_TX_FEE {
        txn.header.fee = MINIMUM_TX_FEE;
      }
      Ok(txn)
    }
  };
}

/// Transaction payment type using the passed in parameters.
/// 
/// - `from` is a checksumed, human readable address for which we register the given participation key.
/// - `fee` is fee per byte is is_flat_fee is false, else it is used as it.
/// - `first_round` is the first round this txn is valid
/// - `last_round` is the last round this txn is valid
/// - `note` is a byte array
/// - `genesis_id` corresponds to the id of the network
/// - `genesis_hash` corresponds to the base64-encoded hash of the genesis of the network
/// - `to` is a checksumed, human readble address of the receipient of the payment
/// - `amount` is the amount to be payment to the receipient in micro algos
/// - `close_remainder_to` if specified, the from account will be closed and the remaining sent to the address specified here.
pub struct PaymentTransactionInput {
  pub from: String,
  pub fee: MicroAlgos,
  pub first_round: Round,
  pub last_round: Round,
  pub note: Option<Vec<u8>>,
  pub genesis_id: String,
  pub genesis_hash: String,
  pub is_flat_fee: bool,
  // payment fields
  pub to: String,
  pub amount: MicroAlgos,
  pub close_remainder_to: Option<String>,
}

impl TransactionInput for PaymentTransactionInput {
  build_header_impl!(TxType::Payment);

  fn build_payment_params(&self) -> Result<Option<PaymentTransactionParams>, Error> {
    let close_remainder_to_addr = if let Some(ref close_address) = self.close_remainder_to {
      Some(Address::from_string(&close_address)?.into())
    } else {
      None
    };
    Ok(Some(
      PaymentTransactionParams {
        receiver: Address::from_string(&self.to)?.into(),
        amount: self.amount,
        close_remainder_to: close_remainder_to_addr,
      }
    ))
  }
}

/// Constructs a keyreg transaction using the fields as parameters
/// 
/// - `from` is a checksumed, human readable address for which we register the given participation key.
/// - `fee` is fee per byte is is_flat_fee is false, else it is used as it.
/// - `first_round` is the first round this txn is valid
/// - `last_round` is the last round this txn is valid
/// - `note` is a byte array
/// - `genesis_id` corresponds to the id of the network
/// - `genesis_hash` corresponds to the base64-encoded hash of the genesis of the network
/// - `vote_pk` is base64 encoded string corresponding to the root participation public key
/// - `selection_pk` is the base64 encoded string corresponding to the vrf public key
/// - `vote_first` is the first round this participation key is valid
/// - `vote_last` is the last round this participation key is valid
/// - `vote_key_dilution` is the dilution for the 2-level pariticpation key
pub struct KeyRegTransactionInput {
  pub from: String,
  pub fee: MicroAlgos,
  pub first_round: Round,
  pub last_round: Round,
  pub note: Option<Vec<u8>>,
  pub genesis_id: String,
  pub genesis_hash: String,
  pub is_flat_fee: bool,
  // keyreg fields
  pub vote_pk: String,
  pub selection_pk: String,
  pub vote_first: Round,
  pub vote_last: Round,
  pub vote_key_dilution: u64,
}

impl TransactionInput for KeyRegTransactionInput {
  build_header_impl!(TxType::KeyReg);

  fn build_key_reg_params(&self) -> Result<Option<KeyRegTransactionParams>, Error> {
    let vote_pk = base64_decode(&self.vote_pk)?;
    let selection_pk = base64_decode(&self.selection_pk)?;
    Ok(Some(
      KeyRegTransactionParams {
        vote_pk: vote_pk.to_array(),
        selection_pk: selection_pk.to_array(),
        vote_first: self.vote_first,
        vote_last: self.vote_last,
        vote_key_dilution: self.vote_key_dilution,
      }
    ))
  }
}

/// Constructs a keyreg transactio using the fields as parameters
/// 
/// - `from` is a checksumed, human readable address for which we register the given participation key.
/// - `fee` is fee per byte is is_flat_fee is false, else it is used as it.
/// - `first_round` is the first round this txn is valid
/// - `last_round` is the last round this txn is valid
/// - `note` is a byte array
/// - `genesis_id` corresponds to the id of the network
/// - `genesis_hash` corresponds to the base64-encoded hash of the genesis of the network
/// - `creator` checksumed address of creator for this asset
/// - `index` index representing the id
/// - `manager` if present should be the checksumed address of the new manager
/// - `reserve` if present should be the checksumed address of account whose holding of this asset is reported as "not minted"
/// - `freeze` if present should be the checksumed address of account allowed to freeze holding of this asset
/// - `clawback` if present should be a valid checksumed address
pub struct AssetConfigTransactionInput {
  pub from: String,
  pub fee: MicroAlgos,
  pub first_round: Round,
  pub last_round: Round,
  pub note: Option<Vec<u8>>,
  pub genesis_id: String,
  pub genesis_hash: String,
  pub is_flat_fee: bool,
  // asset config field
  pub creator: String,
  pub index: u64,
  pub manager: Option<String>,
  pub reserve: Option<String>,
  pub freeze: Option<String>,
  pub clawback: Option<String>,
}

impl TransactionInput for AssetConfigTransactionInput {
  build_header_impl!(TxType::AssetConfig);

  fn build_asset_config_params(&self) -> Result<Option<AssetConfigTransactionParams>, Error> {
    let mut asset_params: AssetParams = Default::default();
    let mut asset_exists = false;
    
    if self.manager.is_some() {
      let vec = Address::from_string(self.manager.as_ref().unwrap())?.to_vec();
      asset_params.manager = Some(ByteBuf::from(vec));
      asset_exists = true;
    }

    if self.reserve.is_some() {
      let vec = Address::from_string(self.reserve.as_ref().unwrap())?.to_vec();
      asset_params.reserve = Some(ByteBuf::from(vec));
      asset_exists = true;
    }

    if self.freeze.is_some() {
      let vec = Address::from_string(self.freeze.as_ref().unwrap())?.to_vec();
      asset_params.freeze = Some(ByteBuf::from(vec));
      asset_exists = true;
    }

    if self.clawback.is_some() {
      let vec = Address::from_string(self.clawback.as_ref().unwrap())?.to_vec();
      asset_params.clawback = Some(ByteBuf::from(vec));
      asset_exists = true;
    }
    

    Ok(Some(
      AssetConfigTransactionParams {
        asset_id: AssetID {
          creator: Address::from_string(&self.creator)?.to_vec(),
          index: self.index,
        },
        asset_params: if asset_exists { Some(asset_params) } else { None },
      }
    ))
  }
}

// TODO(perfectmak): Fix this macro to avoid repeating similar fields for inputs
// macro_rules! transaction_input {
//   ( @ $name:ident { ($($fields:tt)*) } ) => {
//     pub struct $name {
//       pub from: String,
//       pub fee: MicroAlgos,
//       pub first_round: Round,
//       pub last_round: Round,
//       pub note: Vec<u8>,
//       pub genesis_id: String,
//       pub genesis_hash: Vec<u8>,
//       pub is_flat_fee: bool,
//       // type specific field
//       $($result)*
//     }
//   };

//   ( @ $name:ident { $param:ident : $type:ty, $($rest:tt)* } -> ($($result:tt)*) ) => (
//     transaction_input!(@ $name { $($rest)* } -> (
//       $($result)*
//       $param : $type,
//     ));
//   );

//   ( $name:ident { $( $param:ident : $type:ty ),* $(,)* } ) => (
//     transaction_input!(@ $name { $($param : $type,)* } -> ());
//   );
// }

// transaction_input!(PaymentTransactionInput {
//   to: String,
//   amount: MicroAlgos,
//   close_remainder_to: Option<String>,
// });