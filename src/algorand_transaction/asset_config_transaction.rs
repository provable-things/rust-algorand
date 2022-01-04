use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::{
    algorand_address::AlgorandAddress,
    algorand_constants::ALGORAND_MAX_NUM_ROUNDS,
    algorand_hash::AlgorandHash,
    algorand_micro_algos::MicroAlgos,
    algorand_transaction::{transaction_type::AlgorandTransactionType, AlgorandTransaction},
    algorand_types::Result,
};

#[skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct AssetParameters {
    #[serde(rename(serialize = "am"))]
    metadata_hash: AlgorandHash,

    #[serde(rename(serialize = "an"))]
    asset_name: String,

    #[serde(rename(serialize = "au"))]
    asset_url: String,

    /// ## Clawback Address
    ///
    /// The clawback address represents an account that is allowed to transfer assets from and to
    /// any asset holder (assuming they have opted-in). Use this if you need the option to revoke
    /// assets from an account (like if they breach certain contractual obligations tied to holding
    /// the asset). In traditional finance, this sort of transaction is referred to as a clawback.
    #[serde(rename(serialize = "c"))]
    clawback_address: AlgorandAddress,

    #[serde(rename(serialize = "dc"))]
    decimals: u64,

    /// ## Default Frozen
    ///
    /// Whether the asset is created in a froze state.
    #[serde(rename(serialize = "df"))]
    default_frozen: Option<bool>,

    /// ## Freeze Address
    ///
    /// The freeze account is allowed to freeze or unfreeze the asset holdings for a specific
    /// account. When an account is frozen it cannot send or receive the frozen asset. In
    /// traditional finance, freezing assets may be performed to restrict liquidation of company
    /// stock, to investigate suspected criminal activity or to deny-list certain accounts. If the
    /// DefaultFrozen state is set to True, you can use the unfreeze action to authorize certain
    /// accounts to trade the asset (such as after passing KYC/AML checks).
    #[serde(rename(serialize = "f"))]
    freeze_address: AlgorandAddress,

    /// ## Manager Address
    ///
    /// The manager account is the only account that can authorize transactions to re-configure or
    /// destroy an asset.
    #[serde(rename(serialize = "m"))]
    manager_address: AlgorandAddress,

    /// ## Reserve Address
    ///
    /// Specifying a reserve account signifies that non-minted assets will reside in that account
    /// instead of the default creator account. Assets transferred from this account are "minted"
    /// units of the asset. If you specify a new reserve address, you must make sure the new
    /// account has opted into the asset and then issue a transaction to transfer all assets to the
    /// new reserve.
    #[serde(rename(serialize = "r"))]
    reserve_address: AlgorandAddress,

    #[serde(rename(serialize = "t"))]
    total_base_units: u64,

    #[serde(rename(serialize = "un"))]
    unit_name: String,
}

impl AssetParameters {
    pub fn new(
        metadata_hash: AlgorandHash,
        asset_name: &str,
        asset_url: &str,
        clawback_address: AlgorandAddress,
        decimals: u64,
        default_frozen: bool,
        freeze_address: AlgorandAddress,
        manager_address: AlgorandAddress,
        reserve_address: AlgorandAddress,
        total_base_units: u64,
        unit_name: &str,
    ) -> Self {
        Self {
            metadata_hash,
            asset_name: asset_name.to_string(),
            asset_url: asset_url.to_string(),
            clawback_address,
            decimals,
            default_frozen: if default_frozen { Some(true) } else { None },
            freeze_address,
            manager_address,
            reserve_address,
            total_base_units,
            unit_name: unit_name.to_string(),
        }
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
                "Last valid round of {} is > {} away from first valid round of {}!",
                calculated_last_valid_round, ALGORAND_MAX_NUM_ROUNDS, first_valid_round
            )
            .into());
        };
        Ok(Self {
            sender,
            genesis_hash,
            first_valid_round,
            asset_parameters: Some(asset_parameters),
            fee: fee.check_if_satisfies_minimum_fee()?.0,
            last_valid_round: calculated_last_valid_round,
            txn_type: AlgorandTransactionType::AssetConfiguration,
            note: None,
            group: None,
            lease: None,
            amount: None,
            asset_id: None,
            rekey_to: None,
            receiver: None,
            genesis_id: None,
            asset_amount: None,
            asset_receiver: None,
            transfer_asset_id: None,
            close_remainder_to: None,
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
            AlgorandHash::mainnet_genesis_hash().unwrap(),
            "Test Token",
            "google.com",
            sender.clone(),
            18,
            false,
            sender.clone(),
            sender.clone(),
            sender.clone(),
            1_000_000,
            "tTKN",
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
