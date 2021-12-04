use std::result;

use crate::errors::AppError;

pub type Byte = u8;
pub type Bytes = Vec<Byte>;
pub type MicroAlgos = u64;
pub type Result<T> = result::Result<T, AppError>;
