use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;

use crate::{algorand_errors::AlgorandError, algorand_types::Result};

#[skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct AlgorandSignatureJson {
    pub sig: Option<String>,
}

impl AlgorandSignatureJson {
    pub fn is_empty(&self) -> bool {
        self.sig.is_none()
    }
}

impl FromStr for AlgorandSignatureJson {
    // TODO Could impl these with a macro for the all the jsons
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}

impl Display for AlgorandSignatureJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", json!(self))
    }
}
