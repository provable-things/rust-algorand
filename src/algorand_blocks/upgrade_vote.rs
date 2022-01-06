use serde::{Deserialize, Serialize};
use serde_json;
use serde_with::skip_serializing_none;

use crate::algorand_types::Result;

#[skip_serializing_none]
#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct UpgradeVote {
    #[serde(rename = "upgradeprop")]
    upgrade_propose: Option<String>,

    #[serde(rename = "upgradedelay")]
    upgrade_delay: Option<u64>,

    #[serde(rename = "upgradeyes")]
    upgrade_approve: Option<bool>,
}

impl UpgradeVote {
    fn from_json(json: UpgradeVoteJson) -> Self {
        Self {
            upgrade_delay: json.upgrade_delay,
            upgrade_propose: json.upgrade_propose,
            upgrade_approve: json.upgrade_approve,
        }
    }

    pub fn from_str(s: &str) -> Result<Self> {
        UpgradeVoteJson::from_str(s).map(Self::from_json)
    }
}

#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct UpgradeVoteJson {
    #[serde(rename = "upgrade-propose")]
    upgrade_propose: Option<String>,

    #[serde(rename = "upgrade-delay")]
    upgrade_delay: Option<u64>,

    #[serde(rename = "upgrade-approve")]
    upgrade_approve: Option<bool>,
}

impl UpgradeVoteJson {
    pub fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}
