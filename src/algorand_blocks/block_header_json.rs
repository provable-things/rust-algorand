use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;

use crate::{
    algorand_address::AlgorandAddress,
    algorand_blocks::{
        participation_updates::ParticipationUpdatesJson,
        rewards_state::RewardsStateJson,
        upgrade_state::UpgradeStateJson,
        upgrade_vote::UpgradeVoteJson,
    },
    algorand_compact_certificates::compact_certificate_state::CompactCertificateStateJson,
    algorand_errors::AlgorandError,
    algorand_hash::AlgorandHash,
    algorand_micro_algos::MicroAlgos,
    algorand_types::{Byte, Bytes, Result},
    crypto_utils::sha512_256_hash_bytes,
};

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct AlgorandBlockHeaderJson {
    #[serde(rename = "compact-certificates")]
    pub compact_certificates: Option<CompactCertificateStateJson>,

    #[serde(rename = "genesis-hash")]
    pub genesis_hash: Option<String>,

    #[serde(rename = "genesis-id")]
    pub genesis_id: Option<String>,

    #[serde(rename = "previous-block-hash")]
    pub previous_block_hash: Option<String>,

    pub rewards: Option<RewardsStateJson>,

    pub round: u64,

    pub seed: Option<String>,

    pub timestamp: i64,

    #[serde(rename = "transactions-root")]
    pub transactions_root: Option<String>,

    #[serde(rename = "txn-counter")]
    pub transactions_counter: u64,

    #[serde(rename = "upgrade-state")]
    pub upgrade_state: Option<UpgradeStateJson>,

    #[serde(rename = "upgrade-vote")]
    pub upgrade_vote: Option<UpgradeVoteJson>,

    #[serde(rename = "participation-updates")]
    pub participation_updates: Option<ParticipationUpdatesJson>,
}

impl FromStr for AlgorandBlockHeaderJson {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}

impl Display for AlgorandBlockHeaderJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", json!(self))
    }
}

// TODO Impl Display!

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorand_blocks::test_utils::get_sample_block_header_json_n;

    #[test]
    fn should_serde_block_header_to_and_from_str() {
        let header_json = get_sample_block_header_json_n(0);
        let s = header_json.to_string();
        let result = AlgorandBlockHeaderJson::from_str(&s).unwrap();
        assert_eq!(result, header_json);
    }
}
