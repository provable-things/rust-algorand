use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use base64::decode as base64_decode;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{algorand_errors::AlgorandError, algorand_hash::AlgorandHash, algorand_types::Result};

#[derive(Clone, Debug, PartialEq, Eq, EnumIter)]
pub enum AlgorandGenesisId {
    Mainnet,
    Testnet,
    Betanet,
}

impl AlgorandGenesisId {
    fn to_base_64_encoding(&self) -> String {
        match self {
            Self::Mainnet => "wGHE2Pwdvd7S12BL5FaOP20EGYesN73ktiC1qzkkit8=".to_string(),
            Self::Testnet => "SGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiI=".to_string(),
            Self::Betanet => "mFgazF+2uRS1tMiL9dsj01hJGySEmPN28B/TjjvpVW0=".to_string(),
        }
    }

    pub fn hash(&self) -> Result<AlgorandHash> {
        AlgorandHash::from_bytes(&base64_decode(self.to_base_64_encoding())?)
    }

    pub fn from_hash(needle: &AlgorandHash) -> Result<Self> {
        let maybe_self = Self::get_all_as_hashes()?.iter().zip(Self::get_all()).fold(
            None,
            |mut acc, (hash, id)| {
                if needle == hash {
                    acc = Some(id);
                    acc
                } else {
                    acc
                }
            },
        );
        match maybe_self {
            Some(id) => Ok(id),
            None => Err(format!("No Algorand Genesis ID has hash {needle}!").into()),
        }
    }

    fn get_all() -> Vec<Self> {
        Self::iter().collect()
    }

    fn get_all_as_hashes() -> Result<Vec<AlgorandHash>> {
        Self::get_all().iter().map(|x| x.hash()).collect()
    }
}

impl FromStr for AlgorandGenesisId {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_ref() {
            "testnet-v1.0" | "testnet" => Ok(Self::Testnet),
            "mainnet-v1.0" | "mainnet" => Ok(Self::Mainnet),
            "betanet-v1.0" | "betanet" => Ok(Self::Betanet),
            _ => Err(format!("Unrecognized Algorand genesis ID: '{s}'").into()),
        }
    }
}

impl Display for AlgorandGenesisId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mainnet => write!(f, "mainnet-v1.0"),
            Self::Testnet => write!(f, "testnet-v1.0"),
            Self::Betanet => write!(f, "betanet-v1.0"),
        }
    }
}

impl Default for AlgorandGenesisId {
    fn default() -> Self {
        Self::Mainnet
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorand_errors::AlgorandError;

    #[test]
    fn should_get_genesis_id_from_hash() {
        let hashes = AlgorandGenesisId::get_all_as_hashes().unwrap();
        let results = hashes
            .iter()
            .map(|x| AlgorandGenesisId::from_hash(&x))
            .collect::<Result<Vec<AlgorandGenesisId>>>()
            .unwrap();
        AlgorandGenesisId::get_all()
            .iter()
            .enumerate()
            .for_each(|(i, id)| assert_eq!(id, &results[i]));
    }

    #[test]
    fn should_err_when_getting_id_from_unrecognized_hash() {
        let hash = AlgorandHash::default();
        let expected_error = format!("No Algorand Genesis ID has hash {}!", hash);
        match AlgorandGenesisId::from_hash(&hash) {
            Ok(_) => panic!("Should not have succeeded!"),
            Err(AlgorandError::Custom(error)) => assert_eq!(error, expected_error),
            Err(_) => panic!("Wrong error received!"),
        }
    }
}
