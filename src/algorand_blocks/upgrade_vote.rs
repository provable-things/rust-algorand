use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json;
use serde_with::skip_serializing_none;

use crate::{algorand_types::Result, errors::AppError};

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
}

impl FromStr for UpgradeVote {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self> {
        UpgradeVoteJson::from_str(s).map(|ref json| Self::from_json(json))
    }
}

#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct UpgradeVoteJson {
    #[serde(rename = "upgrade-propose")]
    pub upgrade_propose: Option<String>,

    #[serde(rename = "upgrade-delay")]
    pub upgrade_delay: Option<u64>,

    #[serde(rename = "upgrade-approve")]
    pub upgrade_approve: Option<bool>,
}

impl FromStr for UpgradeVoteJson {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}
