use crate::{
    algorand_applications::algorand_application_args::AlgorandApplicationArg,
    algorand_traits::ToApplicationArg,
};

pub struct AlgorandAppId(pub i32);

impl ToApplicationArg for AlgorandAppId {
    fn to_application_arg(&self) -> AlgorandApplicationArg {
        unimplemented!();
    }
}
