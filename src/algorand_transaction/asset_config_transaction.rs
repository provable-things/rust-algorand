use derive_more::Constructor;
use serde::Serialize;

use crate::{
    algorand_hash::AlgorandHash,
    algorand_address::AlgorandAddress,
    algorand_transaction::{
        transaction_type::AlgorandTransactionType,

    },
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
