use base64::decode as base64_decode;

use crate::{
    crypto_utils::base32_encode,
    types::{Byte, Result},
};

const ALGORAND_HASH_NUM_BYTES: usize = 32;

/// ## AlgorandHash
///
/// Stuct to hold the Algorand Hash type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlgorandHash([Byte; ALGORAND_HASH_NUM_BYTES]);

impl AlgorandHash {
    /// ## From Slice
    ///
    /// Construct an AlgorandHash type from a slice of bytes. Errors if number of bytes are not the
    /// expected amount.
    pub fn from_slice(bytes: &[Byte]) -> Result<Self> {
        let number_of_bytes = bytes.len();
        if number_of_bytes != ALGORAND_HASH_NUM_BYTES {
            Err(format!(
                "Not enough bytes to create hash from slice! Got {}, expected {}.",
                number_of_bytes, ALGORAND_HASH_NUM_BYTES
            )
            .into())
        } else {
            Ok(Self(bytes.try_into()?))
        }
    }

    /// ## From Base 64
    ///
    /// Creates the AlgorandHash struct from a base-64 encoded string.
    pub fn from_base_64(s: &str) -> Result<Self> {
        Self::from_slice(&base64_decode(s)?)
    }

    /// ## To Base 64
    ///
    /// Converts the AlgorandHash to it's base-64 encoded counterpart.
    fn to_base_64(&self) -> String {
        base32_encode(&self.0)
    }

    #[cfg(test)]
    fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{errors::AppError, types::Bytes};

    fn get_sample_32_bytes_of_hex() -> &'static str {
        "3832653882a0719ef4f2973a593cd5e062eb4dcd5351c4017a7fd8216327fc51"
    }

    fn get_sample_32_bytes() -> Bytes {
        hex::decode(&get_sample_32_bytes_of_hex()).unwrap()
    }

    fn get_sample_algorand_hash() -> AlgorandHash {
        AlgorandHash::from_slice(&get_sample_32_bytes()).unwrap()
    }

    #[test]
    fn should_get_hash_from_slice() {
        let bytes = get_sample_32_bytes();
        let hash = AlgorandHash::from_slice(&bytes).unwrap();
        let result = hash.0.to_vec();
        assert_eq!(result, bytes);
    }

    #[test]
    fn should_error_when_getting_algorand_hash_from_wrong_number_of_bytes() {
        let mut bytes = get_sample_32_bytes();
        bytes.push(0u8);
        let number_of_bytes = bytes.len();
        let expected_error = format!(
            "Not enough bytes to create hash from slice! Got {}, expected {}.",
            number_of_bytes, ALGORAND_HASH_NUM_BYTES
        );
        match AlgorandHash::from_slice(&bytes) {
            Ok(_) => panic!("Should not have succeeded!"),
            Err(AppError::Custom(error)) => assert_eq!(error, expected_error),
            Err(_) => panic!("Wrong error received!"),
        }
    }

    #[test]
    fn should_convert_algorand_hash_to_hex() {
        let bytes = get_sample_32_bytes();
        let hash = AlgorandHash::from_slice(&bytes).unwrap();
        let result = hash.to_hex();
        assert_eq!(result, get_sample_32_bytes_of_hex());
    }

    #[test]
    fn should_get_algorand_hash_from_base_64_encoding() {
        let genesis_hash = "wGHE2Pwdvd7S12BL5FaOP20EGYesN73ktiC1qzkkit8=";
        let result = AlgorandHash::from_base_64(genesis_hash).unwrap().to_hex();
        let expected_result = "c061c4d8fc1dbdded2d7604be4568e3f6d041987ac37bde4b620b5ab39248adf";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_complete_base_64_encoding_round_trip() {
        let hash = get_sample_algorand_hash();
        let base_64_encoded = hash.to_base_64();
        let result = AlgorandHash::from_base_64(&base_64_encoded).unwrap();
        assert_eq!(result, hash);
    }
}