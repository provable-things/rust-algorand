use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use base64::decode as base64_decode;

use crate::{
    algorand_address::AlgorandAddress,
    algorand_errors::AlgorandError,
    algorand_hash::AlgorandHash,
    algorand_types::Result,
};

pub enum AlgorandGenesisId {
    AlgorandMainnet,
    AlgorandTestnet,
    AlgorandBetanet,
}

impl AlgorandGenesisId {
    fn to_base_64_encoding(&self) -> String {
        match self {
            Self::AlgorandMainnet => "wGHE2Pwdvd7S12BL5FaOP20EGYesN73ktiC1qzkkit8=".to_string(),
            Self::AlgorandTestnet => "SGO1GKSzyE7IEPItTxCByw9x8FmnrCDexi9/cOUJOiI=".to_string(),
            Self::AlgorandBetanet => "mFgazF+2uRS1tMiL9dsj01hJGySEmPN28B/TjjvpVW0=".to_string(),
        }
    }

    pub fn hash(&self) -> Result<AlgorandHash> {
        AlgorandHash::from_bytes(&base64_decode(self.to_base_64_encoding())?)
    }
}

impl FromStr for AlgorandGenesisId {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_ref() {
            "testnet-v1.0" | "testnet" => Ok(Self::AlgorandTestnet),
            "mainnet-v1.0" | "mainnet" => Ok(Self::AlgorandMainnet),
            "betanet-v1.0" | "betanet" => Ok(Self::AlgorandBetanet),
            _ => Err(format!("Unrecognized Algorand genesis ID: '{s}'").into()),
        }
    }
}

impl Display for AlgorandGenesisId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlgorandMainnet => write!(f, "mainnet-v1.0"),
            Self::AlgorandTestnet => write!(f, "testnet-v1.0"),
            Self::AlgorandBetanet => write!(f, "betanet-v1.0"),
        }
    }
}
