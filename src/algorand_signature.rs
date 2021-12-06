use std::fmt;

use base64::encode as base64_encode;
use serde::{Serialize, Serializer};

use crate::{
    crypto_utils::base32_encode_with_no_padding,
    algorand_types::{Byte, Bytes, Result},
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
}

impl fmt::Display for AlgorandSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", base64_encode(&self.0))
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
