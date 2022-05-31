use derive_more::{Constructor, Deref};
use serde::{Deserialize, Serialize};

use crate::algorand_types::Bytes;

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
