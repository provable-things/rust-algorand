#![cfg(test)]
use std::fs::read_to_string;

use paste::paste;

use crate::{algorand_blocks::block::AlgorandBlockHeader, errors::AppError};

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
    0 => "src/algorand_blocks/test_utils/block-17962555.json",
    1 => "src/algorand_blocks/test_utils/block-17962556.json"
);

pub fn get_sample_block_json_str_n(n: usize) -> String {
    read_to_string(get_path_n(n).unwrap()).unwrap()
}

pub fn get_sample_block_header_n(n: usize) -> AlgorandBlockHeader {
    AlgorandBlockHeader::from_str(&get_sample_block_json_str_n(n)).unwrap()
}

mod tests {
    use super::*;

    #[test]
    fn should_get_sample_block_json_n() {
        get_sample_block_json_str_n(0);
    }

    #[test]
    fn should_get_sample_block_n() {
        get_sample_block_header_n(0);
    }
}
