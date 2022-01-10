use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{algorand_types::Result, errors::AppError};

#[skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AlgorandTransactionJson {
    #[serde(rename = "tx-type")]
    tx_type: String,
}

impl FromStr for AlgorandTransactionJson {
    type Err = AppError;

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
            AlgorandTransactionJson::from_str(tx_json_str).unwrap();
        });
    }
}
