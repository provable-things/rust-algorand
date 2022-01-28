use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;

use crate::{
    algorand_errors::AlgorandError,
    algorand_transactions::{
        asset_config_transaction::AssetConfigTransactionJson,
        asset_freeze_transaction::AssetFreezeTransactionJson,
    },
    algorand_types::Result,
};

#[skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct AlgorandTransactionJson {
    #[serde(rename = "asset-freeze-transaction")]
    pub asset_freeze_transaction: Option<AssetFreezeTransactionJson>,

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

impl Display for AlgorandTransactionJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", json!(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorand_transactions::test_utils::{
        get_sample_txs_json_strs_n,
        get_sample_txs_jsons,
    };

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

    #[test]
    fn should_serde_transaction_jsons_to_str() {
        let txs = get_sample_txs_jsons(0);
        let strs = txs.iter().map(|tx| tx.to_string()).collect::<Vec<String>>();
        let results = strs
            .iter()
            .map(|tx_str| AlgorandTransactionJson::from_str(&tx_str))
            .collect::<Result<Vec<AlgorandTransactionJson>>>()
            .unwrap();
        results.iter().enumerate().for_each(|(i, tx)| {
            if *tx != txs[i] {
                println!("{}", tx);
                assert!(false, "Tx does not match original tx!");
            }
        });
    }
}
