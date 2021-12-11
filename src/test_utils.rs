#![cfg(test)]

use crate::{
    algorand_address::AlgorandAddress,
    algorand_keys::AlgorandKeys,
    algorand_mnemonic::AlgorandMnemonic,
    algorand_types::Bytes,
};

pub fn get_sample_private_key_bytes() -> Bytes {
    hex::decode("39564e488e19cdaf66684e06e285afa18ea3cb9f6e9e129d2d97379002b5f86e").unwrap()
}

pub fn get_sample_algorand_keys() -> AlgorandKeys {
    AlgorandKeys::from_bytes(&get_sample_private_key_bytes()).unwrap()
}

pub fn get_sample_algorand_address() -> AlgorandAddress {
    AlgorandKeys::from_bytes(&get_sample_private_key_bytes())
        .unwrap()
        .to_address()
        .unwrap()
}

pub fn get_sample_mnemonic() -> AlgorandMnemonic {
    AlgorandMnemonic::from_str("shrimp deer category ocean olive program drip example dolphin bleak style tube either very insane oyster pelican reopen slide address ahead coil jelly about gossip").unwrap()
}
