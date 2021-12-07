use base64::encode as base64_encode;
use serde::{Serialize, Serializer};

use crate::{
    algorand_types::{Byte, Bytes, Result},
    crypto_utils::{base32_decode, base32_encode_with_no_padding},
};

pub const ALGORAND_PUB_KEY_NUM_BYTES: usize = 32;
pub const ALGORAND_CHECKSUM_NUM_BYTES: usize = 4;
pub const ALGORAND_ADDRESS_BASE_32_NUM_BYTES: usize = 58;

const ALGORAND_ADDRESS_NUM_BYTES: usize = ALGORAND_PUB_KEY_NUM_BYTES + ALGORAND_CHECKSUM_NUM_BYTES;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlgorandAddress([Byte; ALGORAND_ADDRESS_NUM_BYTES]);

// TODO doc commments
impl AlgorandAddress {
    /// ## From Bytes
    ///
    /// Construct an AlgorandAddress from a slice of bytes. Errors if number of bytes are not the
    /// expected amount.
    // TODO validate the checksum?
    pub fn from_bytes(bytes: &[Byte]) -> Result<Self> {
        let number_of_bytes = bytes.len();
        if number_of_bytes != ALGORAND_ADDRESS_NUM_BYTES {
            Err(format!(
                "Not enough bytes to create `AlgorandAddress`! Got {}, expected {}.",
                number_of_bytes, ALGORAND_ADDRESS_NUM_BYTES
            )
            .into())
        } else {
            Ok(Self(bytes.try_into()?))
        }
    }

    pub fn from_str(s: &str) -> Result<Self> {
        base32_decode(s).and_then(|ref bytes| Self::from_bytes(bytes))
    }

    fn to_bytes(&self) -> Bytes {
        self.0.to_vec()
    }

    pub fn to_base64(&self) -> String {
        base64_encode(self.to_pub_key_bytes())
    }

    pub fn to_base32(&self) -> String {
        base32_encode_with_no_padding(&self.0)
    }

    fn to_pub_key_bytes(&self) -> Bytes {
        self.0[..ALGORAND_PUB_KEY_NUM_BYTES].to_vec()
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
        write!(f, "{}", self.to_base32())
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
    fn to_string_should_work() {
        let address = get_sample_algorand_address();
        let result = address.to_string();
        let expected_result = get_sample_algorand_address_str().to_string();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn address_should_make_bytes_roundtrip() {
        let address = get_sample_algorand_address();
        let bytes = address.to_bytes();
        let result = AlgorandAddress::from_bytes(&bytes).unwrap();
        assert_eq!(result, address);
    }

    #[test]
    fn should_get_address_as_base_32() {
        let address = get_sample_algorand_address();
        let result = address.to_base32();
        let expected_result = get_sample_algorand_address_str();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_get_address_as_base_64() {
        let address = get_sample_algorand_address();
        let result = address.to_base64();
        let expected_result = "Mqfb383naV2RrEOBUvyQhhf/v525T4Q8JQJo5v4hoKA=";
        assert_eq!(result, expected_result);
    }
}
