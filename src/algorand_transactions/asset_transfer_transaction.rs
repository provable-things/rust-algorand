use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_with::skip_serializing_none;

use crate::{
    algorand_address::AlgorandAddress,
    algorand_hash::AlgorandHash,
    algorand_micro_algos::MicroAlgos,
    algorand_transactions::{
        transaction::AlgorandTransaction,
        transaction_type::AlgorandTransactionType,
    },
    algorand_types::{Bytes, Result},
};

#[skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Constructor)]
pub struct AssetTransferTransactionJson {
    pub amount: Option<JsonValue>,

    #[serde(rename = "asset-id")]
    pub asset_id: Option<u64>,

    pub receiver: Option<String>,

    #[serde(rename = "close-amount")]
    pub close_amount: Option<u64>,

    pub sender: Option<String>,

    #[serde(rename = "close-to")]
    pub close_to: Option<String>,
}

impl AssetTransferTransactionJson {
    pub fn is_empty(&self) -> bool {
        self.amount.is_none()
            && self.asset_id.is_none()
            && self.receiver.is_none()
            && self.close_amount.is_none()
            && self.sender.is_none()
            && self.close_to.is_none()
    }

    pub fn maybe_get_asset_id(&self) -> Option<u64> {
        self.asset_id
    }

    pub fn maybe_get_asset_amount(&self) -> Option<u64> {
        match &self.amount {
            Some(json_value) => {
                // NOTE: For some reason the indexer returns a number > `u64::MAX` when the
                // explorer shows a `u64::MAX` with some decimals.
                // To catch this error, we see if `serde_json` thinks the value is a float, and if
                // so, use `u64::MAX` instead.
                if json_value.is_f64() {
                    Some(u64::MAX)
                } else {
                    json_value.as_u64()
                }
            },
            None => None,
        }
    }

    pub fn maybe_get_asset_sender(&self) -> Option<String> {
        self.sender.clone()
    }

    pub fn maybe_get_asset_close_to(&self) -> Option<String> {
        self.close_to.clone()
    }

    pub fn maybe_get_asset_receiver(&self) -> Option<String> {
        self.receiver.clone()
    }
}

impl AlgorandTransaction {
    /// ## Asset Transfer
    ///
    /// Assets can be transferred between accounts that have opted-in to receiving the asset. These
    /// are analogous to standard payment transactions but for Algorand Standard Assets.
    pub fn asset_transfer(
        asset_id: u64,
        fee: MicroAlgos,
        asset_amount: u64,
        note: Option<Bytes>,
        first_valid_round: u64,
        sender: AlgorandAddress,
        genesis_hash: AlgorandHash,
        last_valid_round: Option<u64>,
        asset_receiver: AlgorandAddress,
    ) -> Result<AlgorandTransaction> {
        let mut tx = Self::default();
        tx.note = note;
        tx.sender = Some(sender);
        tx.genesis_hash = Some(genesis_hash);
        tx.asset_amount = Some(asset_amount);
        tx.transfer_asset_id = Some(asset_id);
        tx.asset_receiver = Some(asset_receiver);
        tx.first_valid_round = Some(first_valid_round);
        tx.fee = Some(fee.check_if_satisfies_minimum_fee()?.0);
        tx.txn_type = Some(AlgorandTransactionType::AssetTransfer);
        tx.last_valid_round = Some(Self::calculate_last_valid_round(
            first_valid_round,
            last_valid_round,
        )?);
        Ok(tx)
    }

    /// Asset Opt In
    ///
    /// Before an account can receive a specific asset it must opt-in to receive it. An opt-in
    /// transaction places an asset holding of 0 into the account and increases its minimum balance
    /// by 100,000 microAlgos. An opt-in transaction is simply an asset transfer with an amount of
    /// 0, both to and from the account opting in.
    pub fn asset_opt_in(
        asset_id: u64,
        fee: MicroAlgos,
        first_valid_round: u64,
        sender: AlgorandAddress,
        genesis_hash: AlgorandHash,
        last_valid_round: Option<u64>,
    ) -> Result<AlgorandTransaction> {
        let mut tx = Self::default();
        tx.asset_amount = None;
        tx.sender = Some(sender.clone());
        tx.asset_receiver = Some(sender);
        tx.genesis_hash = Some(genesis_hash);
        tx.transfer_asset_id = Some(asset_id);
        tx.first_valid_round = Some(first_valid_round);
        tx.fee = Some(fee.check_if_satisfies_minimum_fee()?.0);
        tx.txn_type = Some(AlgorandTransactionType::AssetTransfer);
        tx.last_valid_round = Some(Self::calculate_last_valid_round(
            first_valid_round,
            last_valid_round,
        )?);
        Ok(tx)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::{
        algorand_hash::AlgorandHash,
        test_utils::{get_sample_algorand_address, get_sample_algorand_keys},
    };

    #[test]
    fn should_sign_opt_in_transaction() {
        let tx = AlgorandTransaction::asset_opt_in(
            463265200,
            MicroAlgos(1000),
            17_962_294,
            get_sample_algorand_address(),
            AlgorandHash::mainnet_genesis_hash().unwrap(),
            None,
        )
        .unwrap();
        let result = tx
            .sign(&get_sample_algorand_keys())
            .unwrap()
            .to_hex()
            .unwrap();
        let expected_result = "82a3736967c4404e714fa39e98ed14a1220a9fe25efbba3b47ccbf784b28bdff0c5fa1f71d1d289034f1157b97ceb63c0cedb3a6790ba87930f6a7a40d495d41ecde101597cd09a374786e88a461726376c42090826960db089ee5636266600d56a9f41f5d037e5c90a18007e384fc1558603da3666565cd03e8a26676ce01121536a26768c420c061c4d8fc1dbdded2d7604be4568e3f6d041987ac37bde4b620b5ab39248adfa26c76ce0112191ea3736e64c42090826960db089ee5636266600d56a9f41f5d037e5c90a18007e384fc1558603da474797065a56178666572a478616964ce1b9cddb0";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_sign_asset_transfer_tx() {
        let tx = AlgorandTransaction::asset_transfer(
            463265200,
            MicroAlgos(1000),
            100001337,
            None,
            17_962_505,
            AlgorandAddress::from_str("GSKWPLI7YL7OF23F5ET5L7HSFLLJL3F5DUO7AH2HQLOSO4DRRHR76TDQ2I")
                .unwrap(),
            AlgorandHash::mainnet_genesis_hash().unwrap(),
            None,
            AlgorandAddress::from_str("GKT5XX6N45UV3ENMIOAVF7EQQYL77P45XFHYIPBFAJUON7RBUCQPX572TI")
                .unwrap(),
        )
        .unwrap();
        let result = tx
            .sign(&get_sample_algorand_keys())
            .unwrap()
            .to_hex()
            .unwrap();
        let expected_result = "82a3736967c440df49d27ce4b5436e7bee58ffcdbe7ff2fb87b56c96309c997f9b525dcac33a78df3992c0ab4ce92b5da3bd7933b7318fdd15e92043bc6d120047e108e437ab07a374786e89a461616d74ce05f5e639a461726376c42032a7dbdfcde7695d91ac438152fc908617ffbf9db94f843c250268e6fe21a0a0a3666565cd03e8a26676ce01121609a26768c420c061c4d8fc1dbdded2d7604be4568e3f6d041987ac37bde4b620b5ab39248adfa26c76ce011219f1a3736e64c420349567ad1fc2fee2eb65e927d5fcf22ad695ecbd1d1df01f4782dd27707189e3a474797065a56178666572a478616964ce1b9cddb0";
        assert_eq!(result, expected_result);
    }
}
