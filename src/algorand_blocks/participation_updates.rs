use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{
    algorand_address::AlgorandAddress,
    algorand_errors::AlgorandError,
    algorand_types::Result,
};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParticipationUpdates {
    #[serde(rename = "partupdrmv")]
    pub expired_participation_accounts: Option<Vec<AlgorandAddress>>,
}

impl ParticipationUpdates {
    pub fn from_json(json: &ParticipationUpdatesJson) -> Result<Self> {
        Ok(Self {
            expired_participation_accounts: match &json.expired_participation_accounts {
                Some(address_strs) => Some(
                    address_strs
                        .iter()
                        .map(|address_str| AlgorandAddress::from_str(address_str))
                        .collect::<Result<Vec<AlgorandAddress>>>()?,
                ),
                None => None,
            },
        })
    }

    pub fn to_json(&self) -> ParticipationUpdatesJson {
        ParticipationUpdatesJson {
            expired_participation_accounts: match &self.expired_participation_accounts {
                None => None,
                Some(expired_accounts) => {
                    Some(expired_accounts.iter().map(|x| x.to_string()).collect())
                },
            },
        }
    }
}

impl FromStr for ParticipationUpdates {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        ParticipationUpdatesJson::from_str(s).and_then(|json| Self::from_json(&json))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct ParticipationUpdatesJson {
    #[serde(rename = "expired-participation-accounts")]
    pub expired_participation_accounts: Option<Vec<String>>,
}

impl ParticipationUpdatesJson {
    pub fn is_empty(&self) -> bool {
        self.expired_participation_accounts.is_none()
    }
}

impl FromStr for ParticipationUpdatesJson {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}
