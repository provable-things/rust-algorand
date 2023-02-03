use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    algorand_address::AlgorandAddress,
    algorand_constants::ALGORAND_MAX_NUM_ROUNDS,
    algorand_hash::AlgorandHash,
    algorand_micro_algos::MicroAlgos,
    algorand_transactions::{
        asset_parameters::{AssetParameters, AssetParametersJson},
        transaction::AlgorandTransaction,
        transaction_type::AlgorandTransactionType,
    },
    algorand_types::Result,
};

#[skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Constructor)]
pub struct AssetConfigTransactionJson {
    #[serde(rename = "asset-id")]
    pub asset_id: u64,
    pub params: AssetParametersJson,
}

impl AssetConfigTransactionJson {
    pub fn maybe_get_asset_id(&self) -> Option<u64> {
        Some(self.asset_id)
    }
}

impl AlgorandTransaction {
    /// ## New Asset Configuration Transaction
    ///
    /// An AssetConfigTx is used to create an asset, modify certain parameters of an asset, or
    /// destroy an asset.
    pub fn new_asset_configuration_tx(
        fee: MicroAlgos,
        first_valid_round: u64,
        sender: AlgorandAddress,
        genesis_hash: AlgorandHash,
        last_valid_round: Option<u64>,
        asset_parameters: AssetParameters,
    ) -> Result<Self> {
        let calculated_last_valid_round =
            Self::calculate_last_valid_round(first_valid_round, last_valid_round)?;
        if calculated_last_valid_round > first_valid_round + ALGORAND_MAX_NUM_ROUNDS {
            return Err(format!(
                "Last valid round of {calculated_last_valid_round} is > {ALGORAND_MAX_NUM_ROUNDS} away from first valid round of {first_valid_round}!"
            )
            .into());
        };
        Ok(Self {
            sender: Some(sender),
            genesis_hash: Some(genesis_hash),
            asset_parameters: Some(asset_parameters),
            first_valid_round: Some(first_valid_round),
            fee: Some(fee.check_if_satisfies_minimum_fee()?.0),
            last_valid_round: Some(calculated_last_valid_round),
            txn_type: Some(AlgorandTransactionType::AssetConfiguration),
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        algorand_traits::ToMsgPackBytes,
        test_utils::{get_sample_algorand_address, get_sample_algorand_keys},
    };

    #[test]
    fn should_sign_asset_config_transaction_correctly() {
        let fee = MicroAlgos(1000);
        let first_valid = 1000;
        let sender = get_sample_algorand_address();
        let genesis_hash = AlgorandHash::mainnet_genesis_hash().unwrap();
        let last_valid_round = None;
        let asset_parameters = AssetParameters::new(
            Some(AlgorandHash::mainnet_genesis_hash().unwrap()),
            Some("Test Token".to_string()),
            Some("google.com".to_string()),
            Some(sender.clone()),
            18,
            false,
            Some(sender.clone()),
            Some(sender.clone()),
            Some(sender.clone()),
            1_000_000,
            Some("tTKN".to_string()),
        );
        let tx = AlgorandTransaction::new_asset_configuration_tx(
            fee,
            first_valid,
            sender,
            genesis_hash,
            last_valid_round,
            asset_parameters,
        )
        .unwrap();
        let keys = get_sample_algorand_keys();
        let signed_tx = tx.sign(&keys).unwrap();
        let result = hex::encode(signed_tx.to_msg_pack_bytes().unwrap());
        let expected_result = "82a3736967c440c5fdf6bff79e8e2e73c71cc4e512a3290a8fdaadc17d66a909d0893955e830a6acc239faba4b2374668f7b30da2c4636246b998ae85299e63e308947c3dbff0ca374786e87a4617061728aa2616dc420c061c4d8fc1dbdded2d7604be4568e3f6d041987ac37bde4b620b5ab39248adfa2616eaa5465737420546f6b656ea26175aa676f6f676c652e636f6da163c42090826960db089ee5636266600d56a9f41f5d037e5c90a18007e384fc1558603da2646312a166c42090826960db089ee5636266600d56a9f41f5d037e5c90a18007e384fc1558603da16dc42090826960db089ee5636266600d56a9f41f5d037e5c90a18007e384fc1558603da172c42090826960db089ee5636266600d56a9f41f5d037e5c90a18007e384fc1558603da174ce000f4240a2756ea474544b4ea3666565cd03e8a26676cd03e8a26768c420c061c4d8fc1dbdded2d7604be4568e3f6d041987ac37bde4b620b5ab39248adfa26c76cd07d0a3736e64c42090826960db089ee5636266600d56a9f41f5d037e5c90a18007e384fc1558603da474797065a461636667";
        assert_eq!(result, expected_result);
    }
}
