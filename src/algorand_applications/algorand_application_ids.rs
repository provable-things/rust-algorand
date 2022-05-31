use derive_more::{Constructor, Deref};

use crate::{
    algorand_applications::algorand_application_args::AlgorandApplicationArg,
    algorand_traits::ToApplicationArg,
};

#[derive(Clone, Debug, Default, Constructor, Deref)]
pub struct AlgorandAppId(pub i32);

impl ToApplicationArg for AlgorandAppId {
    fn to_application_arg(&self) -> AlgorandApplicationArg {
        AlgorandApplicationArg(self.to_be_bytes().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorand_types::Bytes;

    #[test]
    fn should_convert_algorand_app_id_to_app_arg() {
        let app_id = AlgorandAppId::new(1337);
        let result = app_id.to_application_arg().to_hex();
        let expected_result = "00000539";
        assert_eq!(result, expected_result);
    }
}
