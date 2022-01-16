use std::result;

use crate::errors::AlgorandError;

pub type Byte = u8;
pub type Bytes = Vec<Byte>;
pub type Result<T> = result::Result<T, AlgorandError>;
