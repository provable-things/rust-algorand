use std::str::FromStr;

use base64::{decode as base64_decode, encode as base64_encode};
use serde::{Deserialize, Serialize};

use crate::{
    algorand_errors::AlgorandError,
    algorand_types::{Bytes, Result},
    predicates::{is_empty_vec, is_zero_option},
};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct StateProofTracking {
    #[serde(rename = "v", skip_serializing_if = "is_empty_vec")]
    pub voters_commitment: Option<Bytes>,
    #[serde(rename(serialize = "t"), skip_serializing_if = "is_zero_option")]
    pub online_total_weight: Option<u64>,
    #[serde(rename = "n")]
    pub next_round: Option<u64>,
}

impl StateProofTracking {
    pub fn from_json(json: &StateProofTrackingJson) -> Result<Self> {
        Ok(Self {
            voters_commitment: match &json.voters_commitment {
                Some(base64_str) => Some(base64_decode(base64_str)?),
                None => None,
            },
            next_round: json.next_round,
            online_total_weight: json.online_total_weight,
        })
    }

    pub fn to_json(&self, proof_type: u64) -> StateProofTrackingJson {
        StateProofTrackingJson {
            next_round: self.next_round,
            proof_type: Some(proof_type),
            online_total_weight: self.online_total_weight,
            voters_commitment: self.voters_commitment.as_ref().map(base64_encode),
        }
    }
}

impl FromStr for StateProofTracking {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        StateProofTrackingJson::from_str(s).and_then(|json| Self::from_json(&json))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct StateProofTrackingJson {
    #[serde(rename = "next-round", skip_serializing_if = "Option::is_none")]
    pub next_round: Option<u64>,

    #[serde(
        rename = "online-total-weight",
        skip_serializing_if = "Option::is_none"
    )]
    pub online_total_weight: Option<u64>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub proof_type: Option<u64>,

    #[serde(rename = "voters-commitment", skip_serializing_if = "Option::is_none")]
    pub voters_commitment: Option<String>,
}

impl FromStr for StateProofTrackingJson {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}
