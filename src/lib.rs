#![allow(dead_code)] // FIXME rm!
#![allow(clippy::too_many_arguments)]

//! # Rust-Algorand
//!
//! A rust library for building on the Algorand blockchain.
#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate lazy_static;

mod algorand_address;
mod algorand_blocks;
mod algorand_checksum;
mod algorand_compact_certificates;
mod algorand_constants;
mod algorand_encoding;
mod algorand_errors;
mod algorand_genesis_id;
mod algorand_hash;
mod algorand_keys;
mod algorand_micro_algos;
mod algorand_mnemonic;
mod algorand_signature;
mod algorand_traits;
mod algorand_transactions;
mod algorand_types;
mod crypto_utils;
mod test_utils;

pub use crate::{
    algorand_address::AlgorandAddress,
    algorand_blocks::block::AlgorandBlock,
    algorand_errors::AlgorandError,
    algorand_hash::AlgorandHash,
    algorand_keys::AlgorandKeys,
    algorand_micro_algos::MicroAlgos,
    algorand_mnemonic::AlgorandMnemonic,
    algorand_transactions::transaction::AlgorandTransaction,
};
