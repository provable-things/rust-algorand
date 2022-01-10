use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_with::skip_serializing_none;

use crate::{
    algorand_address::AlgorandAddress,
    algorand_blocks::{
        participation_updates::{ParticipationUpdates, ParticipationUpdatesJson},
        rewards_state::{RewardsState, RewardsStateJson},
        upgrade_state::{UpgradeState, UpgradeStateJson},
        upgrade_vote::{UpgradeVote, UpgradeVoteJson},
    },
    algorand_compact_certificates::{
        compact_certificate_state::{CompactCertificateState, CompactCertificateStateJson},
        CompactCertificates,
    },
    algorand_hash::AlgorandHash,
    algorand_micro_algos::MicroAlgos,
    algorand_transaction::AlgorandTransaction,
    algorand_types::{Byte, Bytes, Result},
    crypto_utils::sha512_256_hash_bytes,
    errors::AppError,
};

/*
/// Contains the list of transactions and metadata.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AlgorandBlockHeader {
    #[serde(flatten)]
    header: AlgorandBlockHeader,

    // FIXME This is the block header, THe block itself = header plus txns fields
    #[serde(rename = "transactions")]
    transactions: Option<Vec<AlgorandTransaction>>, // FIXME a type for the vec of tx?
}
*/

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AlgorandBlockHeader {
    #[serde(rename = "earn", default)]
    rewards_level: Option<u64>,

    #[serde(rename = "fees")]
    fee_sink: Option<AlgorandAddress>,

    #[serde(rename = "n")]
    compact_cert_next_round: Option<u64>,

    #[serde(rename = "frac", default)]
    rewards_residue: Option<u64>,

    #[serde(rename = "gen")]
    genesis_id: String,

    #[serde(rename = "gh")]
    genesis_hash: AlgorandHash,

    #[serde(rename = "nextbefore")]
    next_protocol_vote_before: Option<u64>,

    #[serde(rename = "nextproto")]
    next_protocol: Option<String>,

    #[serde(rename = "nextswitch")]
    next_protocol_switch_on: Option<u64>,

    #[serde(rename = "nextyes")]
    next_protocol_approvals: Option<u64>,

    #[serde(rename = "partupdrmv")]
    expired_participation_accounts: Option<Vec<AlgorandAddress>>,

    #[serde(rename = "prev")]
    previous_block_hash: AlgorandHash,

    #[serde(rename = "proto")]
    current_protocol: Option<String>,

    #[serde(rename = "rate", default)]
    rewards_rate: Option<u64>,

    #[serde(rename = "rnd")]
    round: u64,

    #[serde(rename = "rwcalr")]
    rewards_calculation_round: Option<u64>,

    #[serde(rename = "rwd")]
    rewards_pool: Option<AlgorandAddress>,

    seed: AlgorandHash,

    #[serde(rename = "t")]
    compact_cert_voters_total: Option<MicroAlgos>,

    #[serde(rename = "tc")]
    transactions_counter: u64,

    #[serde(rename = "ts")]
    timestamp: i64,

    #[serde(rename = "txn")]
    transactions_root: Option<AlgorandHash>,

    #[serde(rename = "upgradedelay")]
    upgrade_delay: Option<u64>,

    #[serde(rename = "upgradeprop")]
    upgrade_propose: Option<String>,

    #[serde(rename = "upgradeyes")]
    upgrade_approve: Option<bool>,

    #[serde(rename = "v")]
    compact_cert_voters: Option<AlgorandHash>,
}

impl AlgorandBlockHeader {
    fn to_msg_pack_bytes(&self) -> Result<Bytes> {
        Ok(rmp_serde::to_vec_named(&self)?)
    }

    fn prefix_tx_byte(bytes: &[Byte]) -> Bytes {
        let suffix = bytes;
        let mut prefix = b"BH".to_vec();
        prefix.extend_from_slice(suffix);
        prefix
    }

    fn encode_with_prefix(&self) -> Result<Bytes> {
        self.to_msg_pack_bytes()
            .map(|ref msg_pack_bytes| Self::prefix_tx_byte(msg_pack_bytes))
    }

    pub fn hash(&self) -> Result<AlgorandHash> {
        AlgorandHash::from_slice(&sha512_256_hash_bytes(&self.encode_with_prefix()?))
    }

    fn from_json(json: &AlgorandBlockJson) -> Result<Self> {
        Ok(Self {
            genesis_hash: AlgorandHash::from_str(&json.genesis_hash)?,
            genesis_id: json.genesis_id.clone(),
            previous_block_hash: AlgorandHash::from_str(&json.previous_block_hash)?,
            round: json.round,
            seed: AlgorandHash::from_str(&json.seed)?,
            timestamp: json.timestamp,
            transactions_root: match json.transactions_root {
                Some(ref root) => Some(AlgorandHash::from_str(root)?),
                None => None,
            },
            transactions_counter: json.transactions_counter,
            compact_cert_voters: match &json.compact_certificates {
                Some(cert) => Some(AlgorandHash::from_str(&cert.compact_cert_voters)?),
                None => None,
            },
            compact_cert_voters_total: match &json.compact_certificates {
                Some(cert) => Some(MicroAlgos::from_algos(cert.compact_cert_voters_total)?),
                None => None,
            },
            compact_cert_next_round: match &json.compact_certificates {
                Some(cert) => Some(cert.compact_cert_next_round),
                None => None,
            },
            rewards_rate: match &json.rewards {
                Some(rewards) => rewards.rewards_rate.clone(),
                None => None,
            },
            rewards_level: match &json.rewards {
                Some(rewards) => rewards.rewards_level.clone(),
                None => None,
            },
            rewards_residue: match &json.rewards {
                Some(rewards) => rewards.rewards_residue,
                None => None,
            },
            fee_sink: match &json.rewards {
                Some(rewards) => Some(AlgorandAddress::from_str(&rewards.fee_sink)?),
                None => None,
            },
            rewards_pool: match &json.rewards {
                Some(rewards) => Some(AlgorandAddress::from_str(&rewards.rewards_pool)?),
                None => None,
            },
            rewards_calculation_round: match &json.rewards {
                Some(rewards) => Some(rewards.rewards_calculation_round.clone()),
                None => None,
            },
            next_protocol: match &json.upgrade_state {
                Some(upgrade_state) => upgrade_state.next_protocol.clone(),
                None => None,
            },
            current_protocol: match &json.upgrade_state {
                Some(upgrade_state) => Some(upgrade_state.current_protocol.clone()),
                None => None,
            },
            next_protocol_approvals: match &json.upgrade_state {
                None => None,
                Some(upgrade_state) => match upgrade_state.next_protocol_approvals {
                    None | Some(0) => None,
                    Some(thing) => Some(thing),
                },
            },
            next_protocol_switch_on: match &json.upgrade_state {
                None => None,
                Some(upgrade_state) => match upgrade_state.next_protocol_switch_on {
                    None | Some(0) => None,
                    Some(thing) => Some(thing),
                },
            },
            next_protocol_vote_before: match &json.upgrade_state {
                None => None,
                Some(upgrade_state) => match upgrade_state.next_protocol_vote_before {
                    None | Some(0) => None,
                    Some(thing) => Some(thing),
                },
            },
            upgrade_delay: match &json.upgrade_vote {
                None => None,
                Some(upgrade_vote) => match upgrade_vote.upgrade_delay {
                    None | Some(0) => None,
                    Some(thing) => Some(thing),
                },
            },
            upgrade_approve: match &json.upgrade_vote {
                None => None,
                Some(upgrade_vote) => match upgrade_vote.upgrade_approve {
                    None | Some(false) => None,
                    Some(thing) => Some(thing),
                },
            },
            upgrade_propose: match &json.upgrade_vote {
                Some(upgrade_vote) => upgrade_vote.upgrade_propose.clone(),
                None => None,
            },
            expired_participation_accounts: match &json.participation_updates {
                Some(updates) => Some(
                    updates
                        .expired_participation_accounts
                        .iter()
                        .map(|address_str| AlgorandAddress::from_str(address_str))
                        .collect::<Result<Vec<AlgorandAddress>>>()?,
                ),
                None => None,
            },
        })
    }
}

impl FromStr for AlgorandBlockHeader {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self> {
        AlgorandBlockJson::from_str(s).and_then(|ref json| Self::from_json(json))
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AlgorandBlockJson {
    #[serde(rename = "compact-certificates")]
    compact_certificates: Option<CompactCertificateStateJson>,

    #[serde(rename = "genesis-hash")]
    genesis_hash: String,

    #[serde(rename = "genesis-id")]
    genesis_id: String,

    #[serde(rename = "previous-block-hash")]
    previous_block_hash: String,

    rewards: Option<RewardsStateJson>,

    round: u64,

    seed: String,

    timestamp: i64,

    #[serde(rename = "transactions-root")]
    transactions_root: Option<String>,

    #[serde(rename = "txn-counter")]
    transactions_counter: u64,

    #[serde(rename = "upgrade-state")]
    upgrade_state: Option<UpgradeStateJson>,

    #[serde(rename = "upgrade-vote")]
    upgrade_vote: Option<UpgradeVoteJson>,

    #[serde(rename = "participation-updates")]
    participation_updates: Option<ParticipationUpdatesJson>,
}

impl AlgorandBlockJson {
    fn to_str(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl FromStr for AlgorandBlockJson {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorand_blocks::test_utils::{
        get_sample_block_header_n,
        get_sample_block_json_str_n,
    };

    #[test]
    fn should_get_block_header_from_string() {
        let s = get_sample_block_json_str_n(0);
        let result = AlgorandBlockHeader::from_str(&s);
        assert!(result.is_ok());
    }

    #[test]
    fn should_encode_to_msg_pack_bytes() {
        let block = get_sample_block_header_n(0);
        let result = hex::encode(block.to_msg_pack_bytes().unwrap());
        let expected_result = "8fa46561726ece0003474ea466656573c420c7fccdb258f0d4189c2bf8b6d68ee697508642b0ad001f31fcb918c354ba859aa466726163ce3072f41da367656eac6d61696e6e65742d76312e30a26768c420c061c4d8fc1dbdded2d7604be4568e3f6d041987ac37bde4b620b5ab39248adfa470726576c42058fa61ff872ad97805467f38f0620ee8780a9200dc58450cc0c0837d731948d4a570726f746fd95968747470733a2f2f6769746875622e636f6d2f616c676f72616e64666f756e646174696f6e2f73706563732f747265652f62633336303035646264373736653664316561663063353630363139626231383332313536343563a472617465ce029acf20a3726e64ce0112163ba6727763616c72ce0112a880a3727764c420feffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa473656564c420ad0be5fb683c685a096be6217b0963f9aaa4e1af8b69732beff84507da8fbbeaa27463ce1b9d2952a27473ce61b4daaaa374786ec4203308d6d7a61e00a8e5835212291a2c8b83fc8ad35f3d7841f6a8d2faa16042b7";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_get_alogrand_block_header_hash() {
        let block = get_sample_block_header_n(0);
        let message_bytes = hex::encode(block.to_msg_pack_bytes().unwrap());
        let result = block.hash().unwrap();
        let expected_result = get_sample_block_header_n(1).previous_block_hash.clone();
        assert_eq!(result, expected_result);
    }
}
