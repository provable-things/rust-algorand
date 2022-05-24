use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Constructor)]
pub struct KeyRegTransactionJson {
    #[serde(rename = "vote-last-valid")]
    pub vote_last_valid: Option<u64>,

    #[serde(rename = "non-participation")]
    pub non_participation: Option<bool>,

    #[serde(rename = "vote-first-valid")]
    pub vote_first_valid: Option<u64>,

    #[serde(rename = "vote-key-dilution")]
    pub vote_key_dilution: Option<u64>,

    #[serde(rename = "vote-participation-key")]
    pub vote_participation_key: Option<String>,

    #[serde(rename = "selection-participation-key")]
    pub selection_participation_key: Option<String>,
}

impl KeyRegTransactionJson {
    pub fn is_empty(&self) -> bool {
        self.vote_last_valid.is_none()
            && self.non_participation.is_none()
            && self.vote_first_valid.is_none()
            && self.vote_key_dilution.is_none()
            && self.vote_participation_key.is_none()
            && self.selection_participation_key.is_none()
    }
}

// TODO A fxn to make this tx type?
