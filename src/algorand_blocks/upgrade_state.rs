use serde::{Deserialize, Serialize};
use serde_json;
use serde_with::skip_serializing_none;

use crate::algorand_types::Result;

/// Tracks the protocol upgrade state machine.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpgradeState {
    #[serde(rename = "current-protocol")]
    current_protocol: String,

    #[serde(rename = "next-protocol")]
    next_protocol: Option<String>,

    #[serde(rename = "next-protocol-approvals")]
    next_protocol_approvals: Option<u64>,

    #[serde(rename = "next-protocol-vote-before")]
    next_protocol_vote_before: Option<u64>,

    #[serde(rename = "next-protocol-switch-on")]
    next_protocol_switch_on: Option<u64>,
}

impl UpgradeState {
    pub fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}
