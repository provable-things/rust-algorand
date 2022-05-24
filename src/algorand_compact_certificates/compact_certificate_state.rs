use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    algorand_errors::AlgorandError,
    algorand_hash::AlgorandHash,
    algorand_micro_algos::MicroAlgos,
    algorand_types::Result,
};

/// Tracks the state of compact certificates.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CompactCertificateState {
    /// The root of a Merkle tree containing the online accounts that will help
    /// sign a compact certificate. The Merkle root, and the compact certificate,
    /// happen on blocks that are a multiple of ConsensusParams.CompactCertRounds.
    /// For blocks that are not a multiple of ConsensusParams.CompactCertRounds, this value is
    /// zero.
    #[serde(rename = "v")]
    pub compact_cert_voters: Option<AlgorandHash>,

    /// The total number of MicroAlgos held by the accounts in `compact_cert_voters`
    /// (or zero, if the merkle root is zero). This is intended for computing the threshold
    /// of votes to expect from `compact_cert_voters`.
    #[serde(rename = "t")]
    pub compact_cert_voters_total: Option<MicroAlgos>,

    /// The next round for which we will accept a CompactCert transaction.
    #[serde(rename = "n")]
    pub compact_cert_next_round: Option<u64>,
}

impl CompactCertificateState {
    pub fn from_json(json: &CompactCertificateStateJson) -> Result<Self> {
        Ok(Self {
            compact_cert_next_round: json.compact_cert_next_round,
            compact_cert_voters_total: json.compact_cert_voters_total.map(MicroAlgos),
            compact_cert_voters: match &json.compact_cert_voters {
                Some(hash_str) => Some(AlgorandHash::from_str(hash_str)?),
                None => None,
            },
        })
    }

    pub fn to_json(&self) -> CompactCertificateStateJson {
        CompactCertificateStateJson {
            compact_cert_next_round: self.compact_cert_next_round,
            compact_cert_voters_total: self
                .compact_cert_voters_total
                .as_ref()
                .map(|micro_algos| micro_algos.to_algos()),
            compact_cert_voters: self
                .compact_cert_voters
                .as_ref()
                .map(|address| address.to_string()),
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompactCertificateStateJson {
    #[serde(rename = "compact-cert-voters")]
    pub compact_cert_voters: Option<String>,

    #[serde(rename = "compact-cert-next-round")]
    pub compact_cert_next_round: Option<u64>,

    #[serde(rename = "compact-cert-voters-total")]
    pub compact_cert_voters_total: Option<u64>,
}

impl CompactCertificateStateJson {
    pub fn is_empty(&self) -> bool {
        self.compact_cert_voters.is_none()
            && self.compact_cert_next_round.is_none()
            && self.compact_cert_voters_total.is_none()
    }
}

impl FromStr for CompactCertificateStateJson {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}

impl FromStr for CompactCertificateState {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        CompactCertificateStateJson::from_str(s).and_then(|json| Self::from_json(&json))
    }
}
