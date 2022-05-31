use derive_more::{Constructor, Deref};

use crate::{
    algorand_address::AlgorandAddress,
    algorand_applications::algorand_application_args::AlgorandApplicationArg,
    algorand_traits::ToApplicationArg,
    algorand_types::{Byte, Bytes, Result},
    crypto_utils::sha512_256_hash_bytes,
};

#[derive(Clone, Debug, Default, Constructor, Deref)]
pub struct AlgorandAppId(pub i64);

impl ToApplicationArg for AlgorandAppId {
    fn to_application_arg(&self) -> AlgorandApplicationArg {
        AlgorandApplicationArg::from(self.0)
    }
}

impl AlgorandAppId {
    fn to_bytes(&self) -> Bytes {
        self.to_be_bytes().to_vec()
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
}
