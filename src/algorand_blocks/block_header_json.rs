use std::{collections::HashMap, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;

use crate::{
    algorand_blocks::{
        rewards_state::RewardsStateJson,
        state_proof_tracking::{StateProofTracking, StateProofTrackingJson},
        upgrade_state::UpgradeStateJson,
        upgrade_vote::UpgradeVoteJson,
    },
    algorand_compact_certificates::compact_certificate_state::CompactCertificateStateJson,
    algorand_errors::AlgorandError,
    algorand_types::Result,
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

    #[serde(rename = "transactions-root-sha256")]
    pub transactions_root_sha256: Option<String>,

    #[serde(rename = "txn-counter")]
    pub transactions_counter: u64,

    #[serde(rename = "upgrade-state")]
    pub upgrade_state: Option<UpgradeStateJson>,

    #[serde(rename = "upgrade-vote")]
    pub upgrade_vote: Option<UpgradeVoteJson>,

    #[serde(rename = "expired-participation-accounts")]
    pub participation_updates: Option<Vec<String>>,

    #[serde(rename = "state-proof-tracking")]
    pub state_proof_tracking: Option<Vec<StateProofTrackingJson>>,
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

impl AlgorandBlockHeaderJson {
    pub fn maybe_get_state_proof_tracking(&self) -> Option<HashMap<u64, StateProofTracking>> {
        self.state_proof_tracking.as_ref().map(|proofs| {
            let mut hash_map = HashMap::new();
            proofs.iter().for_each(|proof| {
                if let (Some(k), Ok(v)) = (proof.proof_type, StateProofTracking::from_json(proof)) {
                    hash_map.insert(k, v);
                };
            });
            hash_map
        })
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
