use std::{default::Default, str::FromStr};

use base64::encode as base64_encode;
use serde::{Deserialize, Serialize, Serializer};

use crate::{
    algorand_checksum::{AlgorandChecksum, CheckSummableType},
    algorand_keys::AlgorandKeys,
    algorand_types::{Byte, Bytes, Result},
    crypto_utils::{base32_decode, base32_encode_with_no_padding},
    errors::AppError,
};

pub const ALGORAND_ADDRESS_NUM_BYTES: usize = 32;
pub const ALGORAND_ADDRESS_CHECKSUM_NUM_BYTES: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AlgorandAddress([Byte; ALGORAND_ADDRESS_NUM_BYTES]);

impl AlgorandChecksum for AlgorandAddress {
    fn get_check_summable_type() -> CheckSummableType {
        CheckSummableType::AlgorandAddress
    }

    fn to_bytes(&self) -> Result<Bytes> {
        self.to_bytes()
    }

    fn get_checksum_num_bytes() -> usize {
        ALGORAND_ADDRESS_CHECKSUM_NUM_BYTES
    }
}

impl AlgorandAddress {
    /// ## Create Random
    ///
    /// Generate a random Algorand Address
    pub fn create_random() -> Result<Self> {
        AlgorandKeys::create_random().to_address()
    }

    /// ## To Bytes
    ///
    /// Convert the AlgorandAddress to the underlying bytes.
    pub fn to_bytes(&self) -> Result<Bytes> {
        Ok(self.0.to_vec())
    }

    /// ## From Bytes
    ///
    /// Construct an AlgorandAddress from a slice of bytes. Errors if number of bytes are not the
    /// expected amount.
    pub fn from_bytes(bytes: &[Byte]) -> Result<Self> {
        let number_of_bytes = bytes.len();
        if number_of_bytes != ALGORAND_ADDRESS_NUM_BYTES {
            Err(format!(
                "Wrong number of bytes to create `AlgorandAddress`! Got {}, expected {}.",
                number_of_bytes, ALGORAND_ADDRESS_NUM_BYTES
            )
            .into())
        } else {
            Ok(Self(bytes.try_into()?))
        }
    }

    pub fn to_base64(&self) -> Result<String> {
        Ok(base64_encode(&self.append_checksum_bytes()?))
    }

    pub fn to_base32(&self) -> Result<String> {
        Ok(base32_encode_with_no_padding(
            &self.append_checksum_bytes()?,
        ))
    }

    fn to_pub_key_bytes(&self) -> Bytes {
        self.0[..ALGORAND_ADDRESS_NUM_BYTES].to_vec()
    }
}

impl Serialize for AlgorandAddress {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.to_pub_key_bytes())
    }
}

impl std::fmt::Display for AlgorandAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.to_base32() {
            Ok(address) => write!(f, "{}", address),
            Err(_) => write!(f, "Could not get base32 encoding for AlgorandAddress!"),
        }
    }
}

impl Default for AlgorandAddress {
    fn default() -> Self {
        Self([0u8; ALGORAND_ADDRESS_NUM_BYTES])
    }
}

impl FromStr for AlgorandAddress {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self> {
        base32_decode(s)
            .and_then(|ref bytes| Self::from_bytes(&bytes[..ALGORAND_ADDRESS_NUM_BYTES]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_algorand_address_str() -> &'static str {
        "GKT5XX6N45UV3ENMIOAVF7EQQYL77P45XFHYIPBFAJUON7RBUCQPX572TI"
    }

    fn get_sample_algorand_address() -> AlgorandAddress {
        AlgorandAddress::from_str(get_sample_algorand_address_str()).unwrap()
    }

    #[test]
    fn should_get_address_from_str() {
        let s = "GKT5XX6N45UV3ENMIOAVF7EQQYL77P45XFHYIPBFAJUON7RBUCQPX572TI";
        let result = AlgorandAddress::from_str(s);
        assert!(result.is_ok());
    }

    #[test]
    fn address_to_string_should_work() {
        let address = get_sample_algorand_address();
        let result = address.to_string();
        let expected_result = get_sample_algorand_address_str().to_string();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn address_should_make_bytes_roundtrip() {
        let address = get_sample_algorand_address();
        let bytes = address.to_bytes().unwrap();
        let result = AlgorandAddress::from_bytes(&bytes).unwrap();
        assert_eq!(result, address);
    }

    #[test]
    fn should_get_address_as_base_32() {
        let address = get_sample_algorand_address();
        let result = address.to_base32().unwrap();
        let expected_result = get_sample_algorand_address_str();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_get_address_as_base_64() {
        let address = get_sample_algorand_address();
        let result = address.to_base64().unwrap();
        let expected_result = "Mqfb383naV2RrEOBUvyQhhf/v525T4Q8JQJo5v4hoKD79/qa";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_get_default_address() {
        let result = AlgorandAddress::default().to_string();
        let expected_result = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAY5HFKQ";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_create_random_algorand_address() {
        let result = AlgorandAddress::create_random();
        assert!(result.is_ok())
    }
}
