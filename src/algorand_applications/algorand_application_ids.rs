use derive_more::{Constructor, Deref};

use crate::{
    algorand_address::AlgorandAddress,
    algorand_applications::algorand_application_args::AlgorandApplicationArg,
    algorand_traits::ToApplicationArg,
    algorand_types::{Byte, Bytes, Result},
    crypto_utils::sha512_256_hash_bytes,
};

#[derive(Clone, Debug, Eq, PartialEq, Default, Constructor, Deref)]
pub struct AlgorandAppId(pub i64);

impl ToApplicationArg for AlgorandAppId {
    fn to_application_arg(&self) -> AlgorandApplicationArg {
        AlgorandApplicationArg::from(self.0)
    }
}

impl AlgorandAppId {
    pub fn to_bytes(&self) -> Bytes {
        self.to_be_bytes().to_vec()
    }

    pub fn from_bytes(bytes: &[Byte]) -> Result<Self> {
        const I64_NUM_BYTES: usize = 8;
        match bytes.len() {
            0..=7 => Err("✘ Not enough bytes to convert to i64!".into()),
            I64_NUM_BYTES => {
                let mut arr = [0u8; I64_NUM_BYTES];
                let bytes = &bytes[..I64_NUM_BYTES];
                arr.copy_from_slice(bytes);
                Ok(Self::new(i64::from_be_bytes(arr)))
            },
            _ => Err("✘ Too many bytes to convert to i64 without overflowing!".into()),
        }
    }

    fn prefix_app_id_bytes(bytes: &[Byte]) -> Bytes {
        let suffix = bytes;
        let mut prefix = b"appID".to_vec();
        prefix.extend_from_slice(suffix);
        prefix
    }

    fn to_prefixed_bytes(&self) -> Bytes {
        Self::prefix_app_id_bytes(&self.to_bytes())
    }

    pub fn to_address(&self) -> Result<AlgorandAddress> {
        AlgorandAddress::from_bytes(&sha512_256_hash_bytes(&self.to_prefixed_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn should_convert_algorand_app_id_to_app_arg() {
        let app_id = AlgorandAppId::new(1337);
        let result = app_id.to_application_arg().to_hex();
        let expected_result = "0000000000000539";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_get_address_from_app_id() {
        let app_id = AlgorandAppId::new(760689183);
        let expected_result =
            AlgorandAddress::from_str("EIE5DKN2FNN5OTB2VW4YCZGUUZ6SBJBTPD4HR2I4RSPCA7CFJIR72KXVQI")
                .unwrap();
        let result = app_id.to_address().unwrap();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_serde_app_id_to_and_from_bytes() {
        let app_id = AlgorandAppId::new(1337);
        let bytes = app_id.to_bytes();
        let result = AlgorandAppId::from_bytes(&bytes).unwrap();
        assert_eq!(result, app_id);
    }
}
