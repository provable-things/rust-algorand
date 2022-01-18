use rmp_serde::{decode::from_slice as rmp_from_slice, Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    algorand_address::AlgorandAddress,
    algorand_blocks::block_header::AlgorandBlockHeader,
    algorand_hash::AlgorandHash,
    algorand_transactions::transaction::AlgorandTransaction,
    algorand_types::{Byte, Bytes, Result},
};

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AlgorandBlock {
    block_header: AlgorandBlockHeader,
    transactions: Vec<AlgorandTransaction>,
}

impl Default for AlgorandBlock {
    fn default() -> Self {
        Self {
            block_header: AlgorandBlockHeader::default(),
            transactions: vec![],
        }
    }
}

impl AlgorandBlock {
    /// ## To Bytes
    ///
    /// Convert the block to msgpack-ed bytes representation
    pub fn to_bytes(&self) -> Result<Bytes> {
        // TODO Test!
        let mut buffer = Vec::new();
        self.serialize(&mut Serializer::new(&mut buffer)).unwrap();
        Ok(buffer)
    }

    /// ## From Bytes
    ///
    /// Convert a slice of bytes to an AlgorandBlock.
    pub fn from_bytes(bytes: &[Byte]) -> Result<Self> {
        // TODO Test!
        Ok(rmp_from_slice(bytes)?)
    }

    /// ## Hash
    ///
    /// Get the block's header hash
    pub fn hash(&self) -> Result<AlgorandHash> {
        self.block_header.hash()
    }
}
