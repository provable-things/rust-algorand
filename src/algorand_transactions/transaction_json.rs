use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    algorand_errors::AlgorandError,
    algorand_transactions::asset_config_transaction::AssetConfigTransactionJson,
    algorand_types::Result,
};

#[skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AlgorandTransactionJson {
    pub sender: Option<String>,

    pub fee: Option<u64>,

    #[serde(rename = "first-valid")]
    pub first_valid: Option<u64>,

    #[serde(rename = "last-valid")]
    pub last_valid: Option<u64>,

    pub note: Option<String>,

    #[serde(rename = "genesis-id")]
    pub genesis_id: Option<String>,

    #[serde(rename = "genesis-hash")]
    pub genesis_hash: Option<String>,

    #[serde(rename = "tx-type")]
    pub tx_type: Option<String>,

    /// Specifies if this tx is part of a group tx, and if so is the digest of that group.
    pub group: Option<String>,

    /// 32 bytes identifier. While this transaction possesses the lease, no other transaction
    /// specifying this lease can be confirmed.
    pub lease: Option<String>,

    #[serde(rename = "rekey-to")]
    pub rekey_to: Option<String>,

    #[serde(rename = "asset-config-transaction")]
    pub asset_config_transaction: Option<AssetConfigTransactionJson>,

    #[serde(rename = "asset-sender")]
    pub asset_sender: Option<String>,

    #[serde(rename = "asset-receiver")]
    pub asset_receiver: Option<String>,

    pub receiver: Option<String>,

    #[serde(rename = "close-remainder-to")]
    pub close_remainder_to: Option<String>,

    #[serde(rename = "asset-amount")]
    pub asset_amount: Option<u64>,

    #[serde(rename = "transfer-asset-id")]
    pub transfer_asset_id: Option<u64>,

    #[serde(rename = "asset-id")]
    pub asset_id: Option<u64>,

    pub amount: Option<u64>,
}

impl FromStr for AlgorandTransactionJson {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorand_transactions::test_utils::get_sample_txs_json_strs_n;

    #[test]
    fn should_get_txs_from_strs() {
        let txs = get_sample_txs_json_strs_n(0);
        txs.iter().for_each(|tx_json_str| {
            let result = AlgorandTransactionJson::from_str(tx_json_str);
            if result.is_err() {
                println!("{}", tx_json_str);
                result.unwrap();
            }
        });
    }
}
