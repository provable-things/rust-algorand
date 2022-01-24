use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{algorand_errors::AlgorandError, algorand_types::Result};

#[skip_serializing_none]
#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct UpgradeVote {
    #[serde(rename = "upgradeprop")]
    pub upgrade_propose: Option<String>,

    #[serde(rename = "upgradedelay")]
    pub upgrade_delay: Option<u64>,

    #[serde(rename = "upgradeyes")]
    pub upgrade_approve: Option<bool>,
}

impl UpgradeVote {
    pub fn from_json(json: &UpgradeVoteJson) -> Self {
        Self {
            upgrade_delay: match json.upgrade_delay {
                None | Some(0) => None,
                Some(thing) => Some(thing),
            },
            upgrade_approve: match json.upgrade_approve {
                None | Some(false) => None,
                Some(thing) => Some(thing),
            },
            upgrade_propose: json.upgrade_propose.clone(),
        }
    }

    pub fn to_json(&self) -> UpgradeVoteJson {
        UpgradeVoteJson {
            upgrade_delay: self.upgrade_delay,
            upgrade_approve: self.upgrade_approve,
            upgrade_propose: self.upgrade_propose.clone(),
        }
    }
}

impl FromStr for UpgradeVote {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        UpgradeVoteJson::from_str(s).map(|json| Self::from_json(&json))
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct UpgradeVoteJson {
    #[serde(rename = "upgrade-propose")]
    pub upgrade_propose: Option<String>,

    #[serde(rename = "upgrade-delay")]
    pub upgrade_delay: Option<u64>,

    #[serde(rename = "upgrade-approve")]
    pub upgrade_approve: Option<bool>,
}

impl UpgradeVoteJson {
    pub fn is_empty(&self) -> bool {
        self.upgrade_delay.is_none()
            && self.upgrade_propose.is_none()
            && self.upgrade_approve.is_none()
    }
}

impl FromStr for UpgradeVoteJson {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}
