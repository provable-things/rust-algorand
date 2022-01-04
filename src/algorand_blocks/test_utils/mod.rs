#![cfg(test)]

use std::fs::read_to_string;

use crate::errors::AppError;

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

mod tests {
    use super::*;

    #[test]
    fn should_get_sample_block_n() {
        get_sample_block_json_str_n(0);
    }
}
