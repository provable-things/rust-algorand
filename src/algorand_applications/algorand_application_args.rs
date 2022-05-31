use derive_more::{Constructor, Deref};
use serde::{Deserialize, Serialize};

use crate::{
    algorand_address::AlgorandAddress,
    algorand_traits::ToApplicationArg,
    algorand_types::Bytes,
};

#[derive(Default, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Constructor, Deref)]
pub struct AlgorandApplicationArg(#[serde(with = "serde_bytes")] pub Bytes);

impl AlgorandApplicationArg {
    pub fn to_hex(&self) -> String {
        hex::encode(self)
    }
}

impl AsRef<[u8]> for AlgorandApplicationArg {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<&str> for AlgorandApplicationArg {
    fn from(s: &str) -> Self {
        Self::new(s.as_bytes().to_vec())
    }
}

impl From<String> for AlgorandApplicationArg {
    fn from(s: String) -> Self {
        Self::new(s.as_bytes().to_vec())
    }
}

impl From<i64> for AlgorandApplicationArg {
    fn from(n: i64) -> Self {
        Self::new(n.to_be_bytes().to_vec())
    }
}

impl From<u64> for AlgorandApplicationArg {
    fn from(n: u64) -> Self {
        Self::new(n.to_be_bytes().to_vec())
    }
}

impl From<AlgorandAddress> for AlgorandApplicationArg {
    fn from(address: AlgorandAddress) -> Self {
        address.to_application_arg()
    }
}
