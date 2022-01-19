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
    expired_participation_accounts: Vec<AlgorandAddress>,
}

impl ParticipationUpdates {
    pub fn from_json(json: &ParticipationUpdatesJson) -> Result<Self> {
        Ok(Self {
            expired_participation_accounts: json
                .expired_participation_accounts
                .iter()
                .map(|address_str| AlgorandAddress::from_str(address_str))
                .collect::<Result<Vec<AlgorandAddress>>>()?,
        })
    }
}

impl FromStr for ParticipationUpdates {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        ParticipationUpdatesJson::from_str(s).and_then(|json| Self::from_json(&json))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParticipationUpdatesJson {
    #[serde(rename = "expired-participation-accounts")]
    pub expired_participation_accounts: Vec<String>,
}

impl FromStr for ParticipationUpdatesJson {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}
