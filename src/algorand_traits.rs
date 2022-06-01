use serde::Serialize;

use crate::{
    algorand_applications::algorand_application_args::AlgorandApplicationArg,
    algorand_types::{Bytes, Result},
};

/// To Message-Pack Bytes
///
/// A trait to allow a stuct to be encoded into message-pack bytes per the message-pack spec.
pub trait ToMsgPackBytes {
    fn to_msg_pack_bytes(&self) -> Result<Bytes>
    where
        Self: Serialize,
    {
        Ok(rmp_serde::to_vec_named(&self)?)
    }
}

pub trait ToApplicationArg {
    fn to_application_arg(&self) -> AlgorandApplicationArg;
}
