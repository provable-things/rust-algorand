use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use serde_with::skip_serializing_none;

use crate::{
    algorand_errors::AlgorandError,
    algorand_transactions::{
        application_transaction_json::ApplicationTransactionJson,
        asset_config_transaction::AssetConfigTransactionJson,
        asset_freeze_transaction::AssetFreezeTransactionJson,
        asset_transfer_transaction::AssetTransferTransactionJson,
        pay_transaction::PaymentTransactionJson,
        key_reg_transaction::KeyRegTransactionJson,
        signature_json::AlgorandSignatureJson,
        transaction_type::AlgorandTransactionType,
    },
    algorand_types::Result,
};

#[skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct AlgorandTransactionJson {
    #[serde(rename = "asset-freeze-transaction")]
    pub application_transaction: Option<ApplicationTransactionJson>,

    #[serde(rename = "asset-freeze-transaction")]
    pub asset_freeze_transaction: Option<AssetFreezeTransactionJson>,

    #[serde(rename = "asset-transfer-transaction")]
    pub asset_transfer_transaction: Option<AssetTransferTransactionJson>,

    #[serde(rename = "payment-transaction")]
    pub payment_transaction: Option<PaymentTransactionJson>,

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

    pub signature: Option<AlgorandSignatureJson>,

    #[serde(rename = "asset-config-transaction")]
    pub asset_config_transaction: Option<AssetConfigTransactionJson>,

    #[serde(rename = "close-remainder-to")]
    pub close_remainder_to: Option<String>,

    #[serde(rename = "key-reg-transaction")]
    pub key_reg_transaction: Option<KeyRegTransactionJson>,

    pub id: Option<String>,
}

impl AlgorandTransactionJson {
    pub fn get_tx_type(&self) -> AlgorandTransactionType {
        // FIXME Finish this!
        // FIXME Test this!
        if self.asset_freeze_transaction.is_some() {
            AlgorandTransactionType::AssetFreeze
        } else if self.asset_transfer_transaction.is_some() {
            AlgorandTransactionType::AssetTransfer
        } else {
            AlgorandTransactionType::Pay
        }
    }

    pub fn maybe_get_config_asset_id(&self) -> Option<u64> {
        // FIXME Test!
        match &self.asset_config_transaction {
            Some(x) => x.maybe_get_asset_id(),
            None => None,
        }
    }

    pub fn maybe_get_asset_sender(&self) -> Option<String> {
        // FIXME Test!
        match self.get_tx_type() {
            // FIXME Other types of tx!
            AlgorandTransactionType::AssetTransfer => match &self.asset_transfer_transaction {
                Some(x) => x.maybe_get_asset_sender(),
                None => None,
            },
            _ => None,
        }
    }

    pub fn maybe_get_asset_receiver(&self) -> Option<String> {
        // FIXME Test!
        match self.get_tx_type() {
            // FIXME Other types of tx!
            AlgorandTransactionType::AssetTransfer => match &self.asset_transfer_transaction {
                Some(x) => x.maybe_get_asset_receiver(),
                None => None,
            },
            _ => None,
        }
    }

    pub fn maybe_get_receiver(&self) -> Option<String> {
        // FIXME Test!
        match self.get_tx_type() {
            // FIXME Other types of tx!
            AlgorandTransactionType::Pay => match &self.payment_transaction {
                Some(x) => x.maybe_get_receiver(),
                None => None,
            },
            _ => None,
        }
    }

    pub fn maybe_get_asset_amount(&self) -> Option<u64> {
        // FIXME Test!
        match self.get_tx_type() {
            AlgorandTransactionType::AssetTransfer => match &self.asset_transfer_transaction {
                Some(x) => x.maybe_get_asset_amount(),
                None => None,
            },
            // FIXME Other types of tx!
            _ => None,
        }
    }

    pub fn maybe_get_amount(&self) -> Option<u64> {
        // FIXME Test!
        match self.get_tx_type() {
            AlgorandTransactionType::Pay => match &self.payment_transaction {
                Some(x) => x.maybe_get_amount(),
                None => None,
            },
            // FIXME Other types of tx!
            _ => None,
        }
    }

    pub fn maybe_get_asset_close_to(&self) -> Option<String> {
        // FIXME Test!
        match self.get_tx_type() {
            AlgorandTransactionType::AssetTransfer => match &self.asset_transfer_transaction {
                Some(x) => x.maybe_get_asset_close_to(),
                None => None,
            },
            // FIXME Other types of tx!
            _ => None,
        }
    }

    pub fn maybe_get_transfer_asset_id(&self) -> Option<u64> {
        // FIXME Test!
        match &self.asset_transfer_transaction {
            Some(x) => x.maybe_get_asset_id(),
            None => None,
        }
    }
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
impl AlgorandTransactionJson {
    // TODO Macro to get all fields in struct, then I can quickly impl this!
    pub fn assert_equality(&self, other: &AlgorandTransactionJson) {
        use paste::paste;
        let mut err: String;
        macro_rules! assert_equality {
            ($($field:expr),*) => {
                paste! {
                    $(
                        err = format!("'{}' field  does not match!", $field);
                        assert_eq!(self.[< $field >], other.[< $field >], "{}", err);
                    )*
                }
            }
        }
        assert_equality!(
            "asset_config_transaction",
            "asset_freeze_transaction",
            "asset_transfer_transaction",
            "payment_transaction",
            "sender",
            "fee",
            "first_valid",
            "last_valid",
            "note",
            "genesis_id",
            "genesis_hash",
            "tx_type",
            "group",
            "lease",
            "rekey_to",
            "close_remainder_to"
        );
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
                assert!(false)
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
                assert!(false, "Tx does not match original tx!");
            }
        });
    }
}
