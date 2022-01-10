#![cfg(test)]
use std::{fs::read_to_string, str::FromStr};

use paste::paste;
use serde_json::Value as JsonValue;

use crate::{
    algorand_address::AlgorandAddress,
    algorand_hash::AlgorandHash,
    algorand_micro_algos::MicroAlgos,
    algorand_transactions::AlgorandTransaction,
    errors::AppError,
};

macro_rules! write_paths_and_getter_fxn {
    ( $( $num:expr => $path:expr ),* ) => {
        paste! {
            $(const [<SAMPLE_BLOCK_ $num>]: &str = $path;)*

            fn get_path_n(n: usize) -> Result<String, AppError> {
                match n {
                    $($num => Ok([<SAMPLE_BLOCK_ $num>].to_string()),)*
                    _ => Err(AppError::Custom(format!("Cannot find sample block num: {}", n).into())),
                }
            }
        }
    }
}

write_paths_and_getter_fxn!(
    0 => "src/algorand_transactions/test_utils/sample-transactions-block-17962555.json"
);

pub fn get_sample_txs_json_strs_n(n: usize) -> Vec<String> {
    serde_json::from_str::<Vec<JsonValue>>(&read_to_string(get_path_n(n).unwrap()).unwrap())
        .unwrap()
        .iter()
        .map(|json_value| json_value.to_string())
        .collect()
}

pub fn get_sample_pay_tx() -> AlgorandTransaction {
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

mod tests {
    use super::*;

    #[test]
    fn should_get_sample_txs_json_strs_n() {
        get_sample_txs_json_strs_n(0);
    }
}
