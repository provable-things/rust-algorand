pub(crate) mod participation_updates;
pub(crate) mod rewards_state;
pub(crate) mod test_utils;
pub(crate) mod upgrade_state;
pub(crate) mod upgrade_vote;

use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_with::skip_serializing_none;

use crate::{
    algorand_address::AlgorandAddress,
    algorand_blocks::{
        participation_updates::ParticipationUpdates,
        rewards_state::RewardsState,
        upgrade_state::UpgradeState,
        upgrade_vote::UpgradeVote,
    },
    algorand_compact_certificates::{
        compact_certificate_state::CompactCertificateState,
        CompactCertificates,
    },
    algorand_hash::AlgorandHash,
    algorand_micro_algos::MicroAlgos,
    algorand_transaction::AlgorandTransaction,
    algorand_types::{Bytes, Result},
};

/// Contains the list of transactions and metadata.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AlgorandBlock {
    #[serde(flatten)]
    header: AlgorandBlockHeader,

    #[serde(rename = "transactions")]
    transactions: Option<Vec<AlgorandTransaction>>, // TODO a type for the vec of tx?
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
struct AlgorandBlockHeader {
    #[serde(rename = "compact-certificates")]
    compact_certificates: Option<CompactCertificates>,

    #[serde(rename = "genesis-hash")]
    genesis_hash: AlgorandHash,

    #[serde(rename = "genesis-id")]
    genesis_id: String,

    #[serde(rename = "previous-block-hash")]
    previous_block_hash: AlgorandHash,

    rewards: RewardsState,

    round: u64,

    /// Contains cryptographic entropy which can be used to determine a committee.
    seed: AlgorandHash,

    timestamp: i64,
    #[serde(rename = "transactions-root")]
    transactions_root: Option<AlgorandHash>,

    #[serde(rename = "txn-counter")]
    transactions_counter: u64,

    #[serde(rename = "upgrade-state")]
    upgrade_state: Option<UpgradeState>,

    #[serde(rename = "upgrade-vote")]
    upgrade_vote: Option<UpgradeVote>,

    #[serde(flatten)]
    participation_updates: Option<ParticipationUpdates>,
}

impl AlgorandBlockHeader {
    fn to_msg_pack_bytes(&self) -> Result<Bytes> {
        Ok(rmp_serde::to_vec_named(&self)?)
    }

    fn from_str(s: &str) -> Result<Self> {
        AlgorandBlockHeaderJson::from_str(s).and_then(|json| json.to_block_header())
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct AlgorandBlockHeaderJson {
    #[serde(rename = "compact-certificates")]
    compact_certificates: Option<Vec<JsonValue>>,

    #[serde(rename = "genesis-hash")]
    genesis_hash: String,

    #[serde(rename = "genesis-id")]
    genesis_id: String,

    #[serde(rename = "previous-block-hash")]
    previous_block_hash: String,

    rewards: JsonValue,

    round: u64,

    seed: String,

    timestamp: i64,
    #[serde(rename = "transactions-root")]
    transactions_root: Option<String>,

    #[serde(rename = "txn-counter")]
    transactions_counter: u64,

    #[serde(rename = "upgrade-state")]
    upgrade_state: Option<JsonValue>,

    #[serde(rename = "upgrade-vote")]
    upgrade_vote: Option<JsonValue>,

    #[serde(rename = "participation-updates")]
    participation_updates: Option<JsonValue>,
}

impl AlgorandBlockHeaderJson {
    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }

    fn to_str(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    fn to_block_header(&self) -> Result<AlgorandBlockHeader> {
        Ok(AlgorandBlockHeader {
            round: self.round,
            timestamp: self.timestamp,
            genesis_id: self.genesis_id.clone(),
            seed: AlgorandHash::from_str(&self.seed)?,
            transactions_counter: self.transactions_counter,
            genesis_hash: AlgorandHash::from_str(&self.genesis_hash)?,
            rewards: RewardsState::from_str(&self.rewards.to_string())?,
            previous_block_hash: AlgorandHash::from_str(&self.previous_block_hash)?,
            participation_updates: match &self.participation_updates {
                Some(updates) => {
                    Result::Ok(Some(ParticipationUpdates::from_str(&updates.to_string())?))
                },
                None => Result::Ok(None),
            }?,
            compact_certificates: match &self.compact_certificates {
                None => Result::Ok(None),
                Some(certs) => {
                    let mut hash_map = HashMap::new();
                    certs
                        .iter()
                        .map(|cert_json| CompactCertificateState::from_str(&cert_json.to_string()))
                        .collect::<Result<Vec<CompactCertificateState>>>()?
                        .iter()
                        .cloned()
                        .enumerate()
                        .for_each(|(i, cert)| {
                            hash_map.insert(i as u64, cert);
                        });
                    Ok(Some(hash_map))
                },
            }?,
            transactions_root: match &self.transactions_root {
                Some(root) => Result::Ok(Some(AlgorandHash::from_str(&root)?)),
                None => Ok(None),
            }?,
            upgrade_state: match &self.upgrade_state {
                Some(state_json) => {
                    Result::Ok(Some(UpgradeState::from_str(&state_json.to_string())?))
                },
                None => Ok(None),
            }?,
            upgrade_vote: match &self.upgrade_vote {
                Some(vote_json) => Result::Ok(Some(UpgradeVote::from_str(&vote_json.to_string())?)),
                None => Ok(None),
            }?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorand_blocks::test_utils::get_sample_block_json_str_n;

    #[test]
    fn should_get_block_header_from_string() {
        let s = get_sample_block_json_str_n(0);
        let result = AlgorandBlockHeader::from_str(&s);
        //assert!(result.is_ok());
        result.unwrap();
    }
}

/*
impl Block {
    pub fn from_message_pack(buf: &[u8]) -> Result<Self> {
        let mut de = rmp_serde::Deserializer::from_read_ref(buf);
        let _block: Block = Block::deserialize(&mut de)?;
        Ok(rmp_serde::from_slice(buf)?)
    }

    /// Encodes Block as MessagePack bytes.
    fn to_message_pack(&self) -> Result<Vec<u8>> {
        let mut len = 24;

        if let Some(ref cc) = self.header.compact_certificates {
            if cc.is_empty() {
                len -= 1;
            }
        } else {
            len -= 1;
        }

        let mut buf = vec![];

        if self.header.rewards_state.rewards_level.unwrap_or_default() != 0 {
            buf.append(&mut vec![0xa4, 0x65, 0x61, 0x72, 0x6e]);
            rmp_serde::encode::write(&mut buf, &self.header.rewards_state.rewards_level)?;
        } else {
            len -= 1;
        }

        if self.header.rewards_state.fees != AlgorandAddress::default() {
            buf.append(&mut vec![0xa4, 0x66, 0x65, 0x65, 0x73]);
            rmp::encode::write_bin(&mut buf, &self.header.rewards_state.fees.0)?;
        } else {
            len -= 1;
        }

        if self.header.rewards_state.rewards_residue.unwrap_or_default() != 0 {
            buf.append(&mut vec![0xa4, 0x66, 0x72, 0x61, 0x63]);
            rmp_serde::encode::write(&mut buf, &self.header.rewards_state.rewards_residue)?;
        } else {
            len -= 1;
        }

        if !self.header.genesis_id.is_empty() {
            buf.append(&mut vec![0xa3, 0x67, 0x65, 0x6e]);
            rmp_serde::encode::write(&mut buf, &self.header.genesis_id)?;
        } else {
            len -= 1;
        }

        if self.header.genesis_hash != AlgorandHash::default() {
            buf.append(&mut vec![0xa2, 0x67, 0x68]);
            rmp::encode::write_bin(&mut buf, &self.header.genesis_hash.0)?;
        } else {
            len -= 1;
        }

        if self.header.upgrade_state.next_protocol_vote_before.unwrap_or_default() != 0 {
            buf.append(&mut vec![
                0xaa, 0x6e, 0x65, 0x78, 0x74, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65,
            ]);
            rmp_serde::encode::write(&mut buf, &self.header.upgrade_state.next_protocol_vote_before)?;
        } else {
            len -= 1;
        }

        if !self
            .header
            .upgrade_state
            .next_protocol
            .clone()
            .unwrap_or_default()
            .is_empty()
        {
            buf.append(&mut vec![0xa9, 0x6e, 0x65, 0x78, 0x74, 0x70, 0x72, 0x6f, 0x74, 0x6f]);
            rmp_serde::encode::write(&mut buf, &self.header.upgrade_state.next_protocol)?;
        } else {
            len -= 1;
        }

        if self.header.upgrade_state.next_protocol_switch_on.unwrap_or_default() != 0 {
            buf.append(&mut vec![
                0xaa, 0x6e, 0x65, 0x78, 0x74, 0x73, 0x77, 0x69, 0x74, 0x63, 0x68,
            ]);
            rmp_serde::encode::write(&mut buf, &self.header.upgrade_state.next_protocol_switch_on)?;
        } else {
            len -= 1;
        }

        if self.header.upgrade_state.next_protocol_approvals.unwrap_or_default() != 0 {
            buf.append(&mut vec![0xa7, 0x6e, 0x65, 0x78, 0x74, 0x79, 0x65, 0x73]);
            rmp_serde::encode::write(&mut buf, &self.header.upgrade_state.next_protocol_approvals)?;
        } else {
            len -= 1;
        }

        if self.header.previous_block_hash != AlgorandHash::default() {
            buf.append(&mut vec![0xa4, 0x70, 0x72, 0x65, 0x76]);
            rmp::encode::write_bin(&mut buf, &self.header.previous_block_hash.0)?;
        } else {
            len -= 1;
        }

        if !self.header.upgrade_state.current_protocol.is_empty() {
            buf.append(&mut vec![0xa5, 0x70, 0x72, 0x6f, 0x74, 0x6f]);
            rmp_serde::encode::write(&mut buf, &self.header.upgrade_state.current_protocol)?;
        } else {
            len -= 1;
        }

        if self.header.rewards_state.rewards_rate.unwrap_or_default() != 0 {
            buf.append(&mut vec![0xa4, 0x72, 0x61, 0x74, 0x65]);
            rmp_serde::encode::write(&mut buf, &self.header.rewards_state.rewards_rate)?;
        } else {
            len -= 1;
        }

        if self.header.round != 0 {
            buf.append(&mut vec![0xa3, 0x72, 0x6e, 0x64]);
            rmp_serde::encode::write(&mut buf, &self.header.round)?;
        } else {
            len -= 1;
        }

        if self.header.rewards_state.rewards_recalculation_round != 0 {
            buf.append(&mut vec![0xa6, 0x72, 0x77, 0x63, 0x61, 0x6c, 0x72]);
            rmp_serde::encode::write(&mut buf, &self.header.rewards_state.rewards_recalculation_round)?;
        } else {
            len -= 1;
        }

        if self.header.rewards_state.rewards_pool != AlgorandAddress::default() {
            buf.append(&mut vec![0xa3, 0x72, 0x77, 0x64]);
            rmp::encode::write_bin(&mut buf, &self.header.rewards_state.rewards_pool.0)?;
        } else {
            len -= 1;
        }

        if self.header.seed != AlgorandHash::default() {
            buf.append(&mut vec![0xa4, 0x73, 0x65, 0x65, 0x64]);
            rmp::encode::write_bin(&mut buf, &self.header.seed.0)?;
        } else {
            len -= 1;
        }

        if self.header.transactions_counter != 0 {
            buf.append(&mut vec![0xa2, 0x74, 0x63]);
            rmp_serde::encode::write(&mut buf, &self.header.transactions_counter)?;
        } else {
            len -= 1;
        }

        if self.header.timestamp != 0 {
            buf.append(&mut vec![0xa2, 0x74, 0x73]);
            rmp_serde::encode::write(&mut buf, &self.header.timestamp)?;
        } else {
            len -= 1;
        }

        if self.header.transactions_root.unwrap_or_default() != AlgorandHash::default() {
            buf.append(&mut vec![0xa3, 0x74, 0x78, 0x6e]);
            rmp::encode::write_bin(&mut buf, &self.header.transactions_root.unwrap().0)?;
        } else {
            len -= 1;
        }

        if let Some(ref txs) = self.transactions {
            if txs.is_empty() {
                len -= 1;
            } else {
                buf.append(&mut vec![0xa4, 0x74, 0x78, 0x6e, 0x73]);
                // TODO: Impl marshal
                rmp_serde::encode::write(&mut buf, "")?;
            }
        } else {
            len -= 1;
        }

        if self.header.upgrade_vote.upgrade_delay.unwrap_or_default() != 0 {
            buf.append(&mut vec![
                0xac, 0x75, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x64, 0x65, 0x6c, 0x61, 0x79,
            ]);
            rmp_serde::encode::write(&mut buf, &self.header.upgrade_vote.upgrade_delay)?;
        } else {
            len -= 1;
        }

        if !self
            .header
            .upgrade_vote
            .upgrade_propose
            .clone()
            .unwrap_or_default()
            .is_empty()
        {
            buf.append(&mut vec![
                0xab, 0x75, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x70, 0x72, 0x6f, 0x70,
            ]);
            rmp_serde::encode::write(&mut buf, &self.header.upgrade_vote.upgrade_propose)?;
        } else {
            len -= 1;
        }

        if self.header.upgrade_vote.upgrade_approve.unwrap_or_default() {
            buf.append(&mut vec![
                0xaa, 0x75, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x79, 0x65, 0x73,
            ]);
            rmp_serde::encode::write(&mut buf, &self.header.upgrade_vote.upgrade_approve)?;
        } else {
            len -= 1;
        }

        let mut bytes = vec![];
        rmp::encode::write_map_len(&mut bytes, len)?;
        bytes.extend_from_slice(&buf);

        Ok(bytes)
    }

    /// Returns block hash as unprefixed string.
    pub fn hash(&self) -> Result<String> {
        use sha2::AlgorandHash;

        let block_bytes = self.to_message_pack()?;
        let mut prefixed_block_bytes = b"BH".to_vec();
        prefixed_block_bytes.extend_from_slice(&block_bytes);

        Ok(data_encoding::BASE32_NOPAD.encode(&Sha512::digest(&prefixed_block_bytes)))
    }

    /// Resurns previous block hash as unprefixed string.
    pub fn previous_block_hash(&self) -> String {
        let previous_block_hash_with_prefix = self.header.previous_block_hash.to_string();
        previous_block_hash_with_prefix
            .strip_prefix("blk-")
            .unwrap_or(&previous_block_hash_with_prefix)
            .to_string()
    }

    /// Validates current block by comparing its hash to the expected one (next block's `prev`).
    ///
    /// # Arguments
    ///
    /// * `previous_block_hash` - A string slice that holds next block's previous block hash
    ///
    /// # Examples
    ///
    /// ```
    /// use algorand_primitives::test_utils::get_sample_block_json_n;
    ///
    /// let block = get_sample_block_json_n(1);
    /// let next_block = get_sample_block_json_n(2);
    /// assert!(block.validate(&next_block.previous_block_hash()).is_ok());
    /// ```
    pub fn validate(&self, previous_block_hash: &str) -> Result<()> {
        if self.hash()? == previous_block_hash {
            Ok(())
        } else {
            Err(AlgorandError::Custom("Block is invalid!"))
        }
    }
}

impl ToString for Block {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl FromStr for Block {
    type Err = AlgorandError;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let value: Value = serde_json::from_str(s)?;
        if value.is_object() {
            let block = value.get("block");
            Ok(serde_json::from_value(block.unwrap_or(&value).clone())?)
        } else {
            Err(AlgorandError::Other(format!("Can't create Algorand block from {}", s)))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{get_sample_block_json_n, get_sample_block_message_pack_n};

    #[test]
    fn parse_block_from_json() {
        let block = get_sample_block_json_n(1);
        let block_json = block.to_string();
        let expected_result = "{\"rnd\":13850313,\"prev\":\"blk-QLCEYQXM6V4257YAMHJQKFEBJ45GQNZDWTZUBKINZAWKQ6HGA7PQ\",\"seed\":\"Jbc9HDKbIkx0Pglt+YIzC+X/Y2I7QiWrseaeeXdKQI0=\",\"ts\":1619749143,\"gen\":\"testnet-v1.0\",\"gh\":\"SGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiI=\",\"fees\":\"A7NMWS3NT3IUDMLVO26ULGXGIIOUQ3ND2TXSER6EBGRZNOBOUIQXHIBGDE\",\"rwd\":\"7777777777777777777777777777777777777777777777777774MSJUVU\",\"earn\":27224,\"rate\":19999960,\"frac\":4663856440,\"rwcalr\":14000000,\"proto\":\"https://github.com/algorandfoundation/specs/tree/ac2255d586c4474d4ebcf3809acccb59b7ef34ff\",\"tc\":15693792}";
        assert_eq!(block_json, expected_result);
    }

    #[test]
    fn validate_block() {
        let block = get_sample_block_json_n(1);
        let next_block = get_sample_block_json_n(2);
        assert!(block.validate(&next_block.previous_block_hash()).is_ok());
    }

    #[ignore]
    #[test]
    fn parse_block_from_message_pack() {
        let block = get_sample_block_message_pack_n(1);
        let block_json = block.to_string();
        let expected_result = "{\"rnd\":13850313,\"prev\":\"blk-QLCEYQXM6V4257YAMHJQKFEBJ45GQNZDWTZUBKINZAWKQ6HGA7PQ\",\"seed\":\"Jbc9HDKbIkx0Pglt+YIzC+X/Y2I7QiWrseaeeXdKQI0=\",\"ts\":1619749143,\"gen\":\"testnet-v1.0\",\"gh\":\"SGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiI=\",\"fees\":\"A7NMWS3NT3IUDMLVO26ULGXGIIOUQ3ND2TXSER6EBGRZNOBOUIQXHIBGDE\",\"rwd\":\"7777777777777777777777777777777777777777777777777774MSJUVU\",\"earn\":27224,\"rate\":19999960,\"frac\":4663856440,\"rwcalr\":14000000,\"proto\":\"https://github.com/algorandfoundation/specs/tree/ac2255d586c4474d4ebcf3809acccb59b7ef34ff\",\"tc\":15693792}";
        assert_eq!(block_json, expected_result);
    }
}
*/
