#![cfg(test)]
use crate::{
    algorand_address::AlgorandAddress,
    algorand_hash::AlgorandHash,
    algorand_micro_algos::MicroAlgos,
    algorand_transaction::AlgorandTransaction,
};

pub(crate) fn get_sample_pay_tx() -> AlgorandTransaction {
    let first_valid_round = 1000;
    let note = None;
    let last_valid_round = None;
    AlgorandTransaction::new_payment_tx(
        1001337,
        MicroAlgos::minimum_fee(),
        note,
        first_valid_round,
        AlgorandAddress::from_str("4IZRTUO72JY5WH4HKLVDQSKIVF2VSRQX7IFVI3KEOQHHNCQUXCMYPZH7J4")
            .unwrap(),
        AlgorandAddress::from_str("GULDQIEZ2CUPBSHKXRWUW7X3LCYL44AI5GGSHHOQDGKJAZ2OANZJ43S72U")
            .unwrap(),
        AlgorandHash::mainnet_genesis_hash().unwrap(),
        last_valid_round,
    )
    .unwrap()
}
