use std::str::FromStr;

use derive_more::{Constructor, Deref};

use crate::{
    algorand_address::AlgorandAddress,
    algorand_applications::algorand_application_args::AlgorandApplicationArg,
    algorand_errors::AlgorandError,
    algorand_traits::ToApplicationArg,
    algorand_types::{Byte, Bytes, Result},
    crypto_utils::sha512_256_hash_bytes,
};

#[derive(Clone, Debug, Eq, PartialEq, Default, Constructor, Deref)]
pub struct AlgorandAppId(pub u64);

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
        const U64_NUM_BYTES: usize = 8;
        match bytes.len() {
            0..=7 => Err("✘ Not enough bytes to convert to u64!".into()),
            U64_NUM_BYTES => {
                let mut arr = [0u8; U64_NUM_BYTES];
                let bytes = &bytes[..U64_NUM_BYTES];
                arr.copy_from_slice(bytes);
                Ok(Self::new(u64::from_be_bytes(arr)))
            },
            _ => Err("✘ Too many bytes to convert to u64 without overflowing!".into()),
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

impl std::fmt::Display for AlgorandAppId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for AlgorandAppId {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        match s.parse::<u64>() {
            Ok(u_64) => Ok(Self::new(u_64)),
            Err(_) => Err(format!("Cannot convert '{}' to 'AlgorandAppId'!", s).into()),
        }
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
        let app_id = AlgorandAppId::new(760689183);
        let bytes = app_id.to_bytes();
        println!("{}", hex::encode(&bytes));
        let result = AlgorandAppId::from_bytes(&bytes).unwrap();
        assert_eq!(result, app_id);
    }

    #[test]
    fn should_parse_app_id_from_string() {
        let app_id = 1337;
        let s = format!("{}", app_id);
        let result = AlgorandAppId::from_str(&s).unwrap();
        let expected_result = AlgorandAppId::new(app_id);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_fail_to_parse_bad_app_id_from_string() {
        let s = "not an int!";
        let expected_error = format!("Cannot convert '{}' to 'AlgorandAppId'!", s);
        match AlgorandAppId::from_str(&s) {
            Ok(_) => panic!("Should not have succeeded!"),
            Err(AlgorandError::Custom(error)) => assert_eq!(error, expected_error),
            Err(_) => panic!("Wrong error received!"),
        }
    }
}
