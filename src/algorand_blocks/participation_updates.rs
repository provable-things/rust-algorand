use serde::{Deserialize, Serialize};

use crate::{algorand_address::AlgorandAddress, algorand_types::Result};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParticipationUpdates {
    #[serde(rename = "partupdrmv")]
    expired_participation_accounts: Vec<AlgorandAddress>,
}

impl ParticipationUpdates {
    fn from_json(json: ParticipationUpdatesJson) -> Result<Self> {
        Ok(Self {
            expired_participation_accounts: json
                .expired_participation_accounts
                .iter()
                .map(|address_str| AlgorandAddress::from_str(address_str))
                .collect::<Result<Vec<AlgorandAddress>>>()?,
        })
    }

    pub fn from_str(s: &str) -> Result<Self> {
        ParticipationUpdatesJson::from_str(s).and_then(Self::from_json)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct ParticipationUpdatesJson {
    #[serde(rename = "expired-participation-accounts")]
    expired_participation_accounts: Vec<String>,
}

impl ParticipationUpdatesJson {
    pub fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}
