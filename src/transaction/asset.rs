use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

/// AssetID is a name of an asset
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetID {
  #[serde(with = "serde_bytes", rename = "c", skip_serializing_if = "Vec::is_empty")]
  pub creator: Vec<u8>,
  #[serde(rename = "i")]
  pub index: u64,
}

/// AssetParams describes the parameters of an asset
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AssetParams {
  /// asset_name specifies a hint for the name of a unit of this asset
  /// should be 32 bytes long
  #[serde(rename = "an", skip_serializing_if = "Option::is_none")]
  pub asset_name: Option<ByteBuf>,

  /// clawback specifies an account that is allowed to take units
  /// of this asset from any account.
  #[serde(rename = "c", skip_serializing_if = "Option::is_none")]
  pub clawback: Option<ByteBuf>,

  /// default_frozen specifies whether slots for this asset 
  /// in user accounts are frozen by default or not.
  #[serde(rename = "df", skip_serializing_if = "Option::is_none")]
  pub default_frozen: Option<bool>,

  /// freeze specifies an account that is allowed to change the frozen state
  /// of holdings of this asset.
  #[serde(rename = "f", skip_serializing_if = "Option::is_none")]
  pub freeze: Option<ByteBuf>,

  /// manager specifies an account that is allowed to change the
  /// non-zero addresses in this AssetParams
  #[serde(rename = "m", skip_serializing_if = "Option::is_none")]
  pub manager: Option<ByteBuf>,

  /// reserve specifies an account whose holdings of this asset should 
  /// be reported as "not minted".
  #[serde(rename = "r", skip_serializing_if = "Option::is_none")]
  pub reserve: Option<ByteBuf>,

  /// total specifies the total number of units of this asset created
  #[serde(rename = "t", skip_serializing_if = "Option::is_none")]
  pub total: Option<u64>,

  /// unit_name specifies a hint for the name of a unit of this asset
  /// should be 8 bytes long
  #[serde(rename = "un", skip_serializing_if = "Option::is_none")]
  pub unit_name: Option<ByteBuf>,
}

/// Captures the fields used for asset allocation, pre-configuration
/// and destruction.
#[derive(Clone, Debug)]
pub struct AssetConfigTransactionParams {
  /// asset_id is the asset being configured or destroyed.
  pub asset_id: AssetID,

  /// These are params for the asset being created or re-configured.
  pub asset_params: Option<AssetParams>,
}