#![cfg(test)]
use std::{fs::read_to_string, str::FromStr};

use paste::paste;
use serde_json::Value as JsonValue;

use crate::{
    algorand_address::AlgorandAddress,
    algorand_errors::AlgorandError,
    algorand_hash::AlgorandHash,
    algorand_micro_algos::MicroAlgos,
    algorand_transactions::{
        transaction::AlgorandTransaction,
        transaction_json::AlgorandTransactionJson,
    },
    algorand_types::Result,
};

macro_rules! write_paths_and_getter_fxn {
    ( $( $num:expr => $path:expr ),* ) => {
        paste! {
            $(const [<SAMPLE_BLOCK_ $num>]: &str = $path;)*

            fn get_path_n(n: usize) -> Result<String> {
                match n {
                    $($num => Ok([<SAMPLE_BLOCK_ $num>].to_string()),)*
                    _ => Err(AlgorandError::Custom(format!("Cannot find sample block num: {}", n).into())),
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

pub fn get_sample_txs_jsons(n: usize) -> Vec<AlgorandTransactionJson> {
    get_sample_txs_json_strs_n(n)
        .iter()
        .map(|json_str| AlgorandTransactionJson::from_str(json_str))
        .collect::<Result<Vec<AlgorandTransactionJson>>>()
        .unwrap()
}

pub fn get_sample_txs_n(n: usize) -> Vec<AlgorandTransaction> {
    get_sample_txs_jsons(n)
        .iter()
        .map(|tx_json| AlgorandTransaction::from_json(&tx_json))
        .collect::<Result<Vec<AlgorandTransaction>>>()
        .unwrap()
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

pub fn get_sample_acfg_tx_json_string() -> String {
    read_to_string("src/algorand_transactions/test_utils/acfg-tx.json").unwrap()
}

pub fn get_sample_asset_parameters_json_str() -> String {
    use serde::Deserialize;
    #[derive(Deserialize)]
    struct TempStruct {
        #[serde(rename = "asset-config-transaction")]
        asset_config_transaction: TempStructTwo,
    }
    #[derive(Deserialize)]
    struct TempStructTwo {
        params: JsonValue,
    }
    serde_json::from_str::<TempStruct>(&get_sample_acfg_tx_json_string())
        .unwrap()
        .asset_config_transaction
        .params
        .to_string()
}

mod tests {
    use super::*;

    #[test]
    fn should_get_sample_txs_json_strs_n() {
        get_sample_txs_json_strs_n(0);
    }

    #[test]
    fn should_get_sample_tx_jsons_n() {
        get_sample_txs_jsons(0);
    }

    #[test]
    fn should_get_sample_acfg_tx_json_string() {
        get_sample_acfg_tx_json_string();
    }
}
