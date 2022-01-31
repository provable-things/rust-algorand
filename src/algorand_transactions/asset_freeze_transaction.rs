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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Constructor)]
pub struct AssetFreezeTransactionJson {
    pub address: Option<String>,

    #[serde(rename = "asset-id")]
    pub asset_id: Option<u64>,

    #[serde(rename = "new-freeze-status")]
    pub new_freeze_status: Option<bool>,
}

impl AssetFreezeTransactionJson {
    pub fn is_empty(&self) -> bool {
        self.address.is_none() && self.asset_id.is_none() && self.new_freeze_status.is_none()
    }

    pub fn maybe_get_asset_id(&self) -> Option<u64> {
        self.asset_id.clone()
    }
}

/*
impl AlgorandTransaction {
    /// ## New Asset Freeze Transaction
    ///
    /// An Asset Freeze Transaction is issued by the Freeze Address and results in the asset
    /// receiver address losing or being granted the ability to send or receive the frozen asset.
    pub fn new_asset_freeze_tx(
        fee: MicroAlgos,
        first_valid_round: u64,
        sender: AlgorandAddress,
        genesis_hash: AlgorandHash,
        last_valid_round: Option<u64>,
        ???,
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
            sender: Some(sender),
            genesis_hash: Some(genesis_hash),
            asset_parameters: Some(asset_parameters),
            first_valid_round: Some(first_valid_round),
            fee: Some(fee.check_if_satisfies_minimum_fee()?.0),
            last_valid_round: Some(calculated_last_valid_round),
            txn_type: Some(AlgorandTransactionType::AssetConfiguration),
            note: None,
            group: None,
            lease: None,
            amount: None,
            asset_id: None,
            rekey_to: None,
            receiver: None,
            genesis_id: None,
            asset_amount: None,
            asset_sender: None,
            asset_receiver: None,
            asset_freeze_id: None,
            transfer_asset_id: None,
            close_remainder_to: None,
            asset_freeze_address None,
            asset_freeze_status: None,
        })
    }
}
    */

// TODO Test!
