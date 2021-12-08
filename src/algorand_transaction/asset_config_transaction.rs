use derive_more::Constructor;
use serde::Serialize;

use crate::{
    algorand_address::AlgorandAddress,
    algorand_hash::AlgorandHash,
    algorand_micro_algos::MicroAlgos,
    algorand_transaction::{transaction_type::AlgorandTransactionType, AlgorandTransaction},
    algorand_types::{Bytes, Result},
    constants::ALGORAND_MAX_NUM_ROUNDS,
};

#[derive(Debug, Clone, Eq, PartialEq, Constructor, Serialize)]
pub struct AssetParameters {
    #[serde(rename(serialize = "am"))]
    metadata_hash: AlgorandHash,

    #[serde(rename(serialize = "an"))]
    asset_name: String,

    #[serde(rename(serialize = "au"))]
    asset_url: String,

    #[serde(rename(serialize = "c"))]
    clawback_address: AlgorandAddress,

    #[serde(rename(serialize = "dc"))]
    decimals: u64,

    #[serde(rename(serialize = "f"))]
    freeze_address: AlgorandAddress,

    #[serde(rename(serialize = "m"))]
    manager_address: AlgorandAddress,

    #[serde(rename(serialize = "r"))]
    reserve_address: AlgorandAddress,

    #[serde(rename(serialize = "t"))]
    total_base_units: u64,

    #[serde(rename(serialize = "un"))]
    unit_name: String,
}

impl AlgorandTransaction {
    fn new_asset_configuration_tx(
        asset_parameters: AssetParameters,
        amount: u64, // FIXME check this is above 100,000 minimum!
        fee: MicroAlgos,
        note: Option<Bytes>,
        first_valid_round: u64,
        sender: AlgorandAddress,
        receiver: AlgorandAddress,
        genesis_hash: AlgorandHash,
        last_valid_round: Option<u64>,
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
            note,
            sender,
            genesis_hash,
            first_valid_round,
            amount: Some(amount),
            receiver: Some(receiver),
            asset_parameters: Some(asset_parameters),
            fee: fee.check_if_satisfies_minimum_fee()?.0,
            last_valid_round: calculated_last_valid_round,
            txn_type: AlgorandTransactionType::AssetConfiguration,
            group: None,
            lease: None,
            rekey_to: None,
            genesis_id: None,
            close_remainder_to: None,
        })
    }
}
