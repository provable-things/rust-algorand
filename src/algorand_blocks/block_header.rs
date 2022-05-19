use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    algorand_address::AlgorandAddress,
    algorand_blocks::{
        block_header_json::AlgorandBlockHeaderJson,
        participation_updates::ParticipationUpdates,
        rewards_state::RewardsState,
        upgrade_state::UpgradeState,
        upgrade_vote::UpgradeVote,
    },
    algorand_compact_certificates::compact_certificate_state::CompactCertificateState,
    algorand_errors::AlgorandError,
    algorand_hash::AlgorandHash,
    algorand_micro_algos::MicroAlgos,
    algorand_types::{Byte, Bytes, Result},
    crypto_utils::sha512_256_hash_bytes,
};

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlgorandBlockHeader {
    #[serde(rename = "earn")]
    pub rewards_level: Option<u64>,

    #[serde(rename = "fees")]
    pub fee_sink: Option<AlgorandAddress>,

    #[serde(rename = "n")]
    pub compact_cert_next_round: Option<u64>,

    #[serde(rename = "frac", default)]
    pub rewards_residue: Option<u64>,

    #[serde(rename = "gen")]
    pub genesis_id: Option<String>,

    #[serde(rename = "gh")]
    pub genesis_hash: Option<AlgorandHash>,

    #[serde(rename = "nextbefore")]
    pub next_protocol_vote_before: Option<u64>,

    #[serde(rename = "nextproto")]
    pub next_protocol: Option<String>,

    #[serde(rename = "nextswitch")]
    pub next_protocol_switch_on: Option<u64>,

    #[serde(rename = "nextyes")]
    pub next_protocol_approvals: Option<u64>,

    #[serde(rename = "partupdrmv")]
    pub expired_participation_accounts: Option<Vec<AlgorandAddress>>,

    #[serde(rename = "prev")]
    pub previous_block_hash: Option<AlgorandHash>,

    #[serde(rename = "proto")]
    pub current_protocol: Option<String>,

    #[serde(rename = "rate", default)]
    pub rewards_rate: Option<u64>,

    #[serde(rename = "rnd")]
    pub round: u64,

    #[serde(rename = "rwcalr")]
    pub rewards_calculation_round: Option<u64>,

    #[serde(rename = "rwd")]
    pub rewards_pool: Option<AlgorandAddress>,

    pub seed: Option<AlgorandHash>,

    #[serde(rename = "t")]
    pub compact_cert_voters_total: Option<MicroAlgos>,

    #[serde(rename = "tc")]
    pub transactions_counter: u64,

    #[serde(rename = "ts")]
    pub timestamp: i64,

    #[serde(rename = "txn")]
    pub transactions_root: Option<AlgorandHash>,

    #[serde(rename = "upgradedelay")]
    pub upgrade_delay: Option<u64>,

    #[serde(rename = "upgradeprop")]
    pub upgrade_propose: Option<String>,

    #[serde(rename = "upgradeyes")]
    pub upgrade_approve: Option<bool>,

    #[serde(rename = "v")]
    pub compact_cert_voters: Option<AlgorandHash>,
}

impl AlgorandBlockHeader {
    pub fn round(&self) -> u64 {
        self.round
    }

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

    // TODO From bytes!
    /// ## To Bytes
    ///
    /// Convert the block header to a msgpack-ed bytes.
    pub fn to_bytes(&self) -> Result<Bytes> {
        // TODO Test!
        self.to_msg_pack_bytes()
    }

    pub fn hash(&self) -> Result<AlgorandHash> {
        AlgorandHash::from_slice(&sha512_256_hash_bytes(&self.encode_with_prefix()?))
    }

    pub fn from_json(json: &AlgorandBlockHeaderJson) -> Result<Self> {
        Ok(Self {
            // FIXME rm repeat code in this (especially converting the hashes from strs etc)
            genesis_hash: match &json.genesis_hash {
                Some(hash_str) => Some(AlgorandHash::from_str(hash_str)?),
                None => None,
            },
            genesis_id: json.genesis_id.clone(),
            previous_block_hash: match &json.previous_block_hash {
                Some(hash) => Some(AlgorandHash::from_str(hash)?),
                None => None,
            },
            round: json.round,
            seed: match &json.seed {
                Some(hash_str) => Some(AlgorandHash::from_str(hash_str)?),
                None => None,
            },
            timestamp: json.timestamp,
            transactions_root: match json.transactions_root {
                Some(ref root) => Some(AlgorandHash::from_str(root)?),
                None => None,
            },
            transactions_counter: json.transactions_counter,
            compact_cert_voters: match &json.compact_certificates {
                None => None,
                Some(cert) => match &cert.compact_cert_voters {
                    Some(hash_str) => Some(AlgorandHash::from_str(hash_str)?),
                    None => None,
                },
            },
            compact_cert_voters_total: match &json.compact_certificates {
                None => None,
                Some(cert) => match cert.compact_cert_voters_total {
                    None => None,
                    Some(algos) => Some(MicroAlgos::from_algos(algos)?),
                },
            },
            compact_cert_next_round: match &json.compact_certificates {
                Some(certs) => certs.compact_cert_next_round,
                None => None,
            },
            rewards_rate: match &json.rewards {
                Some(rewards) => {
                    if rewards.rewards_rate == Some(0) {
                        None
                    } else {
                        rewards.rewards_rate
                    }
                },
                None => None,
            },
            rewards_level: match &json.rewards {
                Some(rewards) => rewards.rewards_level,
                None => None,
            },
            rewards_residue: match &json.rewards {
                Some(rewards) => rewards.rewards_residue,
                None => None,
            },
            fee_sink: match &json.rewards {
                None => None,
                Some(rewards) => match &rewards.fee_sink {
                    Some(address_string) => Some(AlgorandAddress::from_str(address_string)?),
                    None => None,
                },
            },
            rewards_pool: match &json.rewards {
                Some(rewards) => match &rewards.rewards_pool {
                    Some(address_string) => Some(AlgorandAddress::from_str(address_string)?),
                    None => None,
                },
                None => None,
            },
            rewards_calculation_round: match &json.rewards {
                Some(rewards) => rewards.rewards_calculation_round,
                None => None,
            },
            next_protocol: match &json.upgrade_state {
                Some(upgrade_state) => upgrade_state.next_protocol.clone(),
                None => None,
            },
            current_protocol: match &json.upgrade_state {
                Some(upgrade_state) => upgrade_state.current_protocol.clone(),
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
                Some(x) => ParticipationUpdates::from_json(x)?.expired_participation_accounts,
                None => None,
            },
        })
    }

    fn get_rewards_state(&self) -> RewardsState {
        // TODO Mv this impl to the it's mod to keep this clean!
        RewardsState {
            fee_sink: self.fee_sink.clone(),
            rewards_rate: self.rewards_rate,
            rewards_level: self.rewards_level,
            rewards_residue: self.rewards_residue,
            rewards_pool: self.rewards_pool.clone(),
            rewards_calculation_round: self.rewards_calculation_round,
        }
    }

    fn get_upgrade_state(&self) -> UpgradeState {
        UpgradeState {
            next_protocol: self.next_protocol.clone(),
            current_protocol: self.current_protocol.clone(),
            next_protocol_switch_on: self.next_protocol_switch_on,
            next_protocol_approvals: self.next_protocol_approvals,
            next_protocol_vote_before: self.next_protocol_vote_before,
        }
    }

    fn get_upgrade_vote(&self) -> UpgradeVote {
        UpgradeVote {
            upgrade_delay: self.upgrade_delay,
            upgrade_approve: self.upgrade_approve,
            upgrade_propose: self.upgrade_propose.clone(),
        }
    }

    fn to_participation_updates(&self) -> ParticipationUpdates {
        ParticipationUpdates {
            expired_participation_accounts: self.expired_participation_accounts.clone(),
        }
    }

    fn to_compact_certificate_state(&self) -> CompactCertificateState {
        CompactCertificateState {
            compact_cert_voters: self.compact_cert_voters.clone(),
            compact_cert_next_round: self.compact_cert_next_round,
            compact_cert_voters_total: self.compact_cert_voters_total,
        }
    }

    pub fn to_json(&self) -> Result<AlgorandBlockHeaderJson> {
        let maybe_compact_certificates_state_json = self.to_compact_certificate_state().to_json();
        let compact_certificates = if maybe_compact_certificates_state_json.is_empty() {
            None
        } else {
            Some(maybe_compact_certificates_state_json)
        };
        let maybe_participation_updates_json = self.to_participation_updates().to_json();
        let participation_updates = if maybe_participation_updates_json.is_empty() {
            Some(maybe_participation_updates_json)
        } else {
            None
        };
        let maybe_upgrade_vote_json = self.get_upgrade_vote().to_json();
        let upgrade_vote = if maybe_upgrade_vote_json.is_empty() {
            None
        } else {
            Some(maybe_upgrade_vote_json)
        };
        let maybe_rewards_json = self.get_rewards_state().to_json()?;
        let rewards = if maybe_rewards_json.is_empty() {
            None
        } else {
            Some(maybe_rewards_json)
        };
        let maybe_upgrade_state_json = self.get_upgrade_state().to_json();
        let upgrade_state = if maybe_upgrade_state_json.is_empty() {
            None
        } else {
            Some(maybe_upgrade_state_json)
        };
        Ok(AlgorandBlockHeaderJson {
            rewards,
            upgrade_vote,
            upgrade_state,
            round: self.round,
            compact_certificates,
            participation_updates,
            timestamp: self.timestamp,
            seed: self.seed.as_ref().map(|x| x.to_string()),
            transactions_counter: self.transactions_counter,
            genesis_id: self.genesis_id.as_ref().map(|x| x.to_string()),
            genesis_hash: self.genesis_hash.as_ref().map(|x| x.to_string()),
            transactions_root: self.transactions_root.as_ref().map(|x| x.to_string()),
            previous_block_hash: self.previous_block_hash.as_ref().map(|x| x.to_string()),
        })
    }
}

impl FromStr for AlgorandBlockHeader {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        AlgorandBlockHeaderJson::from_str(s).and_then(|json| Self::from_json(&json))
    }
}

// TODO Impl Display!

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
        let header = get_sample_block_header_n(0);
        let result = hex::encode(header.to_msg_pack_bytes().unwrap());
        let expected_result = "8fa46561726ece0003474ea466656573c420c7fccdb258f0d4189c2bf8b6d68ee697508642b0ad001f31fcb918c354ba859aa466726163ce3072f41da367656eac6d61696e6e65742d76312e30a26768c420c061c4d8fc1dbdded2d7604be4568e3f6d041987ac37bde4b620b5ab39248adfa470726576c42058fa61ff872ad97805467f38f0620ee8780a9200dc58450cc0c0837d731948d4a570726f746fd95968747470733a2f2f6769746875622e636f6d2f616c676f72616e64666f756e646174696f6e2f73706563732f747265652f62633336303035646264373736653664316561663063353630363139626231383332313536343563a472617465ce029acf20a3726e64ce0112163ba6727763616c72ce0112a880a3727764c420feffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa473656564c420ad0be5fb683c685a096be6217b0963f9aaa4e1af8b69732beff84507da8fbbeaa27463ce1b9d2952a27473ce61b4daaaa374786ec4203308d6d7a61e00a8e5835212291a2c8b83fc8ad35f3d7841f6a8d2faa16042b7";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_get_alogrand_block_header_hash() {
        let header = get_sample_block_header_n(0);
        let result = header.hash().unwrap();
        let expected_result = get_sample_block_header_n(1)
            .previous_block_hash
            .unwrap()
            .clone();
        assert_eq!(result, expected_result);
    }
}
