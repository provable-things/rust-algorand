use std::{fmt, str::FromStr};

use base64::{decode as base64_decode, encode as base64_encode};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    algorand_encoding::U8_64Visitor,
    algorand_errors::AlgorandError,
    algorand_types::{Byte, Result},
};

const ALGORAND_SIGNATURE_NUM_BYTES: usize = 64;

/// ## Algorand Signature
///
/// A struct to hold an ED25519 signature, implementing upon it the correct serialization & display
/// methods.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlgorandSignature([Byte; ALGORAND_SIGNATURE_NUM_BYTES]);

impl AlgorandSignature {
    /// ## From Byte Array
    ///
    /// Create the AlgorandSignature struct from an array of 64 bytes.
    pub fn from_byte_array(bytes: [Byte; ALGORAND_SIGNATURE_NUM_BYTES]) -> Self {
        Self(bytes)
    }

    /// ## To Byte Array
    ///
    /// Get the underlying byte-array of the signature.
    pub fn to_byte_array(&self) -> [Byte; ALGORAND_SIGNATURE_NUM_BYTES] {
        self.0
    }

    /// ## To Hex
    ///
    /// Get the signature as a hex string.
    pub fn to_hex(&self) -> String {
        hex::encode(self.to_byte_array())
    }

    pub fn from_slice(bytes: &[Byte]) -> Result<Self> {
        let number_of_bytes = bytes.len();
        if number_of_bytes != ALGORAND_SIGNATURE_NUM_BYTES {
            Err(format!(
                "Not enough bytes to create hash from slice! Got {number_of_bytes}, expected {ALGORAND_SIGNATURE_NUM_BYTES}."
            )
            .into())
        } else {
            Ok(Self(bytes.try_into()?))
        }
    }

    fn from_base_64(s: &str) -> Result<Self> {
        Self::from_slice(&base64_decode(s)?)
    }
}

impl fmt::Display for AlgorandSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", base64_encode(self.0))
    }
}

impl FromStr for AlgorandSignature {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_base_64(s)
    }
}

impl Serialize for AlgorandSignature {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}

impl<'de> Deserialize<'de> for AlgorandSignature {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        Ok(AlgorandSignature(
            deserializer.deserialize_bytes(U8_64Visitor)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::get_sample_algorand_keys;

    fn get_sample_signature() -> AlgorandSignature {
        get_sample_algorand_keys().sign(b"A message")
    }

    #[test]
    fn algorand_signature_should_make_bytes_serde_round_trip() {
        let signature = get_sample_signature();
        let byte_array = signature.to_byte_array();
        let result = AlgorandSignature::from_byte_array(byte_array);
        assert_eq!(result, signature);
    }
}
