use serde::{Deserialize, Serialize};

use crate::algorand_types::Bytes;

#[derive(Default, Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct AlgorandApplicationArg(#[serde(with = "serde_bytes")] pub Bytes);

impl AsRef<[u8]> for AlgorandApplicationArg {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
