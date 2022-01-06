#![cfg(test)]

use std::fs::read_to_string;

use crate::{algorand_blocks::block::AlgorandBlock, errors::AppError};

const SAMPLE_BLOCK_0: &str = "src/algorand_blocks/test_utils/block-17962555.json";

pub fn get_sample_block_json_str_n(n: usize) -> String {
    let path = match n {
        0 => Ok(SAMPLE_BLOCK_0),
        _ => Err(AppError::Custom(
            format!("Cannot find sample block num: {}", n).into(),
        )),
    }
    .unwrap();
    read_to_string(path).unwrap()
}

pub fn get_sample_block_n(n: usize) -> AlgorandBlock {
    AlgorandBlock::from_str(&get_sample_block_json_str_n(n)).unwrap()
}

mod tests {
    use super::*;

    #[test]
    fn should_get_sample_block_json_n() {
        get_sample_block_json_str_n(0);
    }

    #[test]
    fn should_get_sample_block_n() {
        get_sample_block_n(0);
    }
}
