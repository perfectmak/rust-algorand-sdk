// Unit tests for the transactions module

use crate::encoding::base64_decode;
use super::{Transaction, PaymentTransactionInput, KeyRegTransactionInput, AssetConfigTransactionInput};
use crate::accounts::Account;

#[test]
fn test_make_payment_transaction_works() {
  let from_address = "47YPQTIGQEO7T4Y4RWDYWEKV6RTR2UNBQXBABEEGM72ESWDQNCQ52OPASU";
	let to_address = "PNWOET7LLOWMBMLE4KOCELCX6X3D3Q4H2Q4QJASYIEOF7YIPPQBG3YQ5YI";
	let expected_reference_txn_id = "5FJDJD5LMZC3EHUYYJNH5I23U4X6H2KXABNDGPIL557ZMJ33GZHQ";
	let mnemonic = "advice pudding treat near rule blouse same whisper inner electric quit surface sunny dismiss leader blood seat clown cost exist hospital century reform able sponsor";
	let golden = "gqNzaWfEQPhUAZ3xkDDcc8FvOVo6UinzmKBCqs0woYSfodlmBMfQvGbeUx3Srxy3dyJDzv7rLm26BRv9FnL2/AuT7NYfiAWjdHhui6NhbXTNA+ilY2xvc2XEIEDpNJKIJWTLzpxZpptnVCaJ6aHDoqnqW2Wm6KRCH/xXo2ZlZc0EmKJmds0wsqNnZW6sZGV2bmV0LXYzMy4womdoxCAmCyAJoJOohot5WHIvpeVG7eftF+TYXEx4r7BFJpDt0qJsds00mqRub3RlxAjqABVHQ2y/lqNyY3bEIHts4k/rW6zAsWTinCIsV/X2PcOH1DkEglhBHF/hD3wCo3NuZMQg5/D4TQaBHfnzHI2HixFV9GcdUaGFwgCQhmf0SVhwaKGkdHlwZaNwYXk=";
  let expected_signed_bytes = base64_decode(golden).unwrap();
  let close_remainder_to = "IDUTJEUIEVSMXTU4LGTJWZ2UE2E6TIODUKU6UW3FU3UKIQQ77RLUBBBFLA";
  let gh = "JgsgCaCTqIaLeVhyL6XlRu3n7Rfk2FxMeK+wRSaQ7dI=";
  let note = base64_decode("6gAVR0Nsv5Y=").unwrap();
  let account = Account::from_mnemonic(mnemonic).unwrap();

  let txn = Transaction::from_input(PaymentTransactionInput {
    from: from_address.into(),
    to: to_address.into(),
    fee: 4,
    amount: 1000,
    first_round: 12466,
    last_round: 13466,
    note: Some(note),
    close_remainder_to: Some(close_remainder_to.into()),
    genesis_id: "devnet-v33.0".into(),
    genesis_hash: gh.into(),
    is_flat_fee: false,
  }).unwrap();
  
  let signed_txn = txn.sign(&account).unwrap();
  let actual_signed_bytes = signed_txn.encode().unwrap();

  assert_eq!(actual_signed_bytes, expected_signed_bytes);
  assert_eq!(signed_txn.txn_id, expected_reference_txn_id);
}

#[test]
fn test_key_reg_transaction_works() {
  let mnemonic = "advice pudding treat near rule blouse same whisper inner electric quit surface sunny dismiss leader blood seat clown cost exist hospital century reform able sponsor";
	let golden = "gqNzaWfEQEA8ANbrvTRxU9c8v6WERcEPw7D/HacRgg4vICa61vEof60Wwtx6KJKDyvBuvViFeacLlngPY6vYCVP0DktTwQ2jdHhui6NmZWXNA+iiZnbOAATsD6JnaMQgSGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiKibHbOAATv96ZzZWxrZXnEIGz4K7+GKID3HWlAMa7dUMrGGU1ckQLlDA+M0JgrvZZXo3NuZMQgCfvSdiwI+Gxa5r9t16epAd5mdddQ4H6MXHaYZH224f2kdHlwZaZrZXlyZWendm90ZWZzdM0nEKZ2b3Rla2QLp3ZvdGVrZXnEICr+0CO3IYtcumsaMvre8MwFaXj6kav65I81of0TGMi6p3ZvdGVsc3TNJ38=";
  let expected_signed_bytes = base64_decode(golden).unwrap();
  let account = Account::from_mnemonic(mnemonic).unwrap();

  let txn = Transaction::from_input(KeyRegTransactionInput {
    from: "BH55E5RMBD4GYWXGX5W5PJ5JAHPGM5OXKDQH5DC4O2MGI7NW4H6VOE4CP4".into(),
    fee: 10,
    first_round: 322575,
    last_round: 323575,
    note: Some(([45, 67]).to_vec()),
    genesis_id: "".into(),
    genesis_hash: String::from("SGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiI="),
    vote_pk: "Kv7QI7chi1y6axoy+t7wzAVpePqRq/rkjzWh/RMYyLo=".into(),
    selection_pk: "bPgrv4YogPcdaUAxrt1QysYZTVyRAuUMD4zQmCu9llc=".into(),
    vote_first: 10000,
    vote_last: 10111,
    vote_key_dilution: 11,
    is_flat_fee: false,
  }).unwrap();

  println!("Debug {:?}", txn.to_raw());
  let signed_txn = txn.sign(&account).unwrap();
  let actual_signed_bytes = signed_txn.encode().unwrap();

  assert_eq!(actual_signed_bytes, expected_signed_bytes);
  assert_eq!(signed_txn.txn_id, "MDRIUVH5AW4Z3GMOB67WP44LYLEVM2MP3ZEPKFHUB5J47A2J6TUQ");
}

#[test]
fn test_asset_cfg_transaction_works() {
  let address: String = "BH55E5RMBD4GYWXGX5W5PJ5JAHPGM5OXKDQH5DC4O2MGI7NW4H6VOE4CP4".into();
  let mnemonic = "advice pudding treat near rule blouse same whisper inner electric quit surface sunny dismiss leader blood seat clown cost exist hospital century reform able sponsor";
	let golden = "gqNzaWfEQCRiqooONBncRNNplEiW0aKkcOn64MdOlHiRNN81GDQx0SqUYKL1q//4Yi5ziFdmtFOC7Iu/I8qbCkSlYPUVRAWjdHhuiKRhcGFyhKFjxCAJ+9J2LAj4bFrmv23Xp6kB3mZ111Dgfoxcdphkfbbh/aFmxCAJ+9J2LAj4bFrmv23Xp6kB3mZ111Dgfoxcdphkfbbh/aFtxCAJ+9J2LAj4bFrmv23Xp6kB3mZ111Dgfoxcdphkfbbh/aFyxCAJ+9J2LAj4bFrmv23Xp6kB3mZ111Dgfoxcdphkfbbh/aRjYWlkgqFjxCAJ+9J2LAj4bFrmv23Xp6kB3mZ111Dgfoxcdphkfbbh/aFpzQTSo2ZlZc0OzqJmds4ABOwPomdoxCBIY7UYpLPITsgQ8i1PEIHLD3HwWaesIN7GL39w5Qk6IqJsds4ABO/3o3NuZMQg5/D4TQaBHfnzHI2HixFV9GcdUaGFwgCQhmf0SVhwaKGkdHlwZaRhY2Zn";
  let expected_signed_bytes = base64_decode(golden).unwrap();
  let account = Account::from_mnemonic(mnemonic).unwrap();

  let txn = Transaction::from_input(AssetConfigTransactionInput {
    from: address.clone(),
    fee: 10,
    first_round: 322575,
    last_round: 323575,
    note: None,
    genesis_id: String::new(),
    genesis_hash: String::from("SGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiI="),
    creator: address.clone(),
    index: 1234,
    manager: Some(address.clone()),
    reserve: Some(address.clone()),
    freeze: Some(address.clone()),
    clawback: Some(address.clone()),
    is_flat_fee: false,
  }).unwrap();

  println!("Debug {:?}", txn.to_raw());
  let signed_txn = txn.sign(&account).unwrap();
  let actual_signed_bytes = signed_txn.encode().unwrap();

  assert_eq!(actual_signed_bytes, expected_signed_bytes);
}
