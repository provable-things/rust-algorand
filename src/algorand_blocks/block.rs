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
        block_json::AlgorandBlockJson,
    },
    algorand_errors::AlgorandError,
    algorand_hash::AlgorandHash,
    algorand_transactions::{
        transaction::AlgorandTransaction,
        transaction_json::AlgorandTransactionJson,
        transactions::AlgorandTransactions,
    },
    algorand_types::{Byte, Bytes, Result},
};

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlgorandBlock {
    pub block_header: AlgorandBlockHeader,
    pub transactions: Option<AlgorandTransactions>,
}

impl Default for AlgorandBlock {
    fn default() -> Self {
        Self {
            block_header: AlgorandBlockHeader::default(),
            transactions: None,
        }
    }
}

impl AlgorandBlock {
    /// ## Get Previous Block Hash
    ///
    /// Returns the previous block has from the block header
    pub fn get_previous_block_hash(&self) -> Result<AlgorandHash> {
        match &self.block_header.previous_block_hash {
            Some(hash) => Ok(hash.clone()),
            None => Err("Could not get previous block hash from block!".into()),
        }
    }

    /// ## To Bytes
    ///
    /// Convert the block to bytes.
    pub fn to_bytes(&self) -> Result<Bytes> {
        Ok(serde_json::to_vec(&self.to_json()?)?)
    }

    /// ## From Bytes
    ///
    /// Convert a slice of bytes to an AlgorandBlock.
    pub fn from_bytes(bytes: &[Byte]) -> Result<Self> {
        Self::from_json(&serde_json::from_slice::<AlgorandBlockJson>(&bytes)?)
    }

    /// ## Hash
    ///
    /// Get the block's header hash
    pub fn hash(&self) -> Result<AlgorandHash> {
        self.block_header.hash()
    }

    /// ## Round
    ///
    /// Get the round number of the block.
    pub fn round(&self) -> u64 {
        self.block_header.round()
    }

    fn from_json(json: &AlgorandBlockJson) -> Result<Self> {
        let txs = json
            .transactions
            .iter()
            .map(|transaction_json| AlgorandTransaction::from_json(&transaction_json))
            .collect::<Result<Vec<AlgorandTransaction>>>()?;
        Ok(Self {
            block_header: AlgorandBlockHeader::from_json(&json.block_header)?,
            transactions: if json.transactions.is_empty() {
                None
            } else {
                Some(AlgorandTransactions::from_jsons(&json.transactions)?)
            },
        })
    }

    fn to_json(&self) -> Result<AlgorandBlockJson> {
        Ok(AlgorandBlockJson {
            block_header: self.block_header.to_json()?,
            transactions: match &self.transactions {
                None => vec![],
                Some(txs) => txs
                    .iter()
                    .map(|tx| tx.to_json())
                    .collect::<Result<Vec<AlgorandTransactionJson>>>()?,
            },
        })
    }
}

impl std::str::FromStr for AlgorandBlock {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_json(&AlgorandBlockJson::from_str(s)?)
    }
}

impl Display for AlgorandBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.to_json() {
            Ok(json_struct) => write!(f, "{}", json!(json_struct)),
            Err(error) => write!(f, "Could not convert AlgorandBlock to json!: {}", error),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::algorand_blocks::test_utils::{
        get_all_sample_blocks,
        get_sample_block_json_str_n,
        get_sample_block_n,
    };

    #[test]
    fn should_get_block_from_str() {
        let s = get_sample_block_json_str_n(0);
        let result = AlgorandBlock::from_str(&s);
        assert!(result.is_ok());
    }

    #[test]
    fn should_serde_block_to_and_from_string() {
        let block = get_sample_block_n(0);
        let s = block.to_string();
        let result = AlgorandBlock::from_str(&s).unwrap();
        assert_eq!(
            result.block_header.rewards_level,
            block.block_header.rewards_level
        );
        assert_eq!(result.block_header.fee_sink, block.block_header.fee_sink);
        assert_eq!(
            result.block_header.compact_cert_next_round,
            block.block_header.compact_cert_next_round
        );
        assert_eq!(
            result.block_header.rewards_residue,
            block.block_header.rewards_residue
        );
        assert_eq!(
            result.block_header.genesis_id,
            block.block_header.genesis_id
        );
        assert_eq!(
            result.block_header.genesis_hash,
            block.block_header.genesis_hash
        );
        assert_eq!(
            result.block_header.next_protocol_vote_before,
            block.block_header.next_protocol_vote_before
        );
        assert_eq!(
            result.block_header.next_protocol,
            block.block_header.next_protocol
        );
        assert_eq!(
            result.block_header.next_protocol_switch_on,
            block.block_header.next_protocol_switch_on
        );
        assert_eq!(
            result.block_header.next_protocol_approvals,
            block.block_header.next_protocol_approvals
        );
        assert_eq!(
            result.block_header.expired_participation_accounts,
            block.block_header.expired_participation_accounts
        );
        assert_eq!(
            result.block_header.previous_block_hash,
            block.block_header.previous_block_hash
        );
        assert_eq!(
            result.block_header.current_protocol,
            block.block_header.current_protocol
        );
        assert_eq!(
            result.block_header.rewards_rate,
            block.block_header.rewards_rate
        );
        assert_eq!(result.block_header.round, block.block_header.round);
        assert_eq!(
            result.block_header.rewards_calculation_round,
            block.block_header.rewards_calculation_round
        );
        assert_eq!(
            result.block_header.rewards_pool,
            block.block_header.rewards_pool
        );
        assert_eq!(result.block_header.seed, block.block_header.seed);
        assert_eq!(
            result.block_header.compact_cert_voters_total,
            block.block_header.compact_cert_voters_total
        );
        assert_eq!(
            result.block_header.transactions_counter,
            block.block_header.transactions_counter
        );
        assert_eq!(result.block_header.timestamp, block.block_header.timestamp);
        assert_eq!(
            result.block_header.transactions_root,
            block.block_header.transactions_root
        );
        assert_eq!(
            result.block_header.upgrade_delay,
            block.block_header.upgrade_delay
        );
        assert_eq!(
            result.block_header.upgrade_propose,
            block.block_header.upgrade_propose
        );
        assert_eq!(
            result.block_header.upgrade_approve,
            block.block_header.upgrade_approve
        );
        assert_eq!(
            result.block_header.compact_cert_voters,
            block.block_header.compact_cert_voters
        );
    }

    #[test]
    fn should_serde_block_to_and_from_bytes() {
        get_all_sample_blocks()
            .iter()
            .enumerate()
            .for_each(|(i, block)| {
                let bytes = block.to_bytes().unwrap();
                let result = AlgorandBlock::from_bytes(&bytes).unwrap();
                if result != *block {
                    assert!(false, "Block {i} failed equality test assertion!");
                }
            })
    }
}
