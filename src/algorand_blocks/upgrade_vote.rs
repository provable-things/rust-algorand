use serde::{Deserialize, Serialize};
use serde_json;
use serde_with::skip_serializing_none;

use crate::algorand_types::Result;

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpgradeVote {
    #[serde(rename = "upgrade-propose")]
    upgrade_propose: Option<String>,

    #[serde(rename = "upgrade-delay")]
    upgrade_delay: Option<u64>,

    #[serde(rename = "upgrade-approve")]
    upgrade_approve: Option<bool>,
}

impl UpgradeVote {
    pub fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}
