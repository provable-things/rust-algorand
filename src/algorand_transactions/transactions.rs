use derive_more::Deref;
use serde::{Deserialize, Serialize};

use crate::{
    algorand_transactions::{
        transaction::AlgorandTransaction,
        transaction_json::AlgorandTransactionJson,
    },
    algorand_types::Result,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Deref)]
pub struct AlgorandTransactions(Vec<AlgorandTransaction>);

impl AlgorandTransactions {
    pub fn from_jsons(jsons: &[AlgorandTransactionJson]) -> Result<Self> {
        Ok(Self(
            jsons
                .iter()
                .map(|tx_json| AlgorandTransaction::from_json(&tx_json))
                .collect::<Result<Vec<AlgorandTransaction>>>()?,
        ))
    }
}
