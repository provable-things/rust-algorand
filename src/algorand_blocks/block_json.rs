use std::{fmt::Display, str::FromStr};

use rmp_serde::{decode::from_slice as rmp_from_slice, Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;

use crate::{
    algorand_address::AlgorandAddress,
    algorand_blocks::{
        block_header::AlgorandBlockHeader,
        block_header_json::AlgorandBlockHeaderJson,
    },
    algorand_errors::AlgorandError,
    algorand_hash::AlgorandHash,
    algorand_transactions::{
        transaction::AlgorandTransaction,
        transaction_json::AlgorandTransactionJson,
    },
    algorand_types::{Byte, Bytes, Result},
};

#[skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AlgorandBlockJson {
    #[serde(flatten)]
    pub block_header: AlgorandBlockHeaderJson,

    pub transactions: Vec<AlgorandTransactionJson>,
}

impl FromStr for AlgorandBlockJson {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}

impl Display for AlgorandBlockJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", json!(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorand_blocks::test_utils::get_sample_block_json_n;

    #[test]
    fn should_serde_block_json_to_and_from_str() {
        let block_json = get_sample_block_json_n(0);
        let s = block_json.to_string();
        let result = AlgorandBlockJson::from_str(&s).unwrap();
        assert_eq!(result, block_json);
    }
}
