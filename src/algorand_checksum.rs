use crate::{
    algorand_types::{Byte, Bytes, Result},
    crypto_utils::sha512_256_hash_bytes,
};

const ALGORAND_MAX_CHECKSUM_BYTES: usize = 32;

pub enum CheckSummableType {
    AlgorandAddress,
    AlgorandMnemonic,
}

pub trait AlgorandChecksum {
    fn to_bytes(&self) -> Result<Bytes>;
    fn get_checksum_num_bytes() -> usize;
    fn get_check_summable_type() -> CheckSummableType;

    fn check_checksum_num_bytes(checksum_num_bytes: usize) -> Result<usize> {
        if checksum_num_bytes > ALGORAND_MAX_CHECKSUM_BYTES {
            Err(format!(
                "You asked for {checksum_num_bytes} bytes, but you cannot get > {ALGORAND_MAX_CHECKSUM_BYTES} bytes!"
            )
            .into())
        } else {
            Ok(checksum_num_bytes)
        }
    }

    fn calculate_checksum_bytes(
        bytes: &[Byte],
        checksum_num_bytes: usize,
        check_summable_type: CheckSummableType,
    ) -> Result<Bytes> {
        Self::check_checksum_num_bytes(checksum_num_bytes).map(
            |num_bytes| match check_summable_type {
                CheckSummableType::AlgorandMnemonic => {
                    sha512_256_hash_bytes(bytes)[..num_bytes].to_vec()
                },
                CheckSummableType::AlgorandAddress => {
                    sha512_256_hash_bytes(bytes)[32 - num_bytes..32].to_vec()
                },
            },
        )
    }

    fn get_checksum_bytes(&self) -> Result<Bytes> {
        Self::calculate_checksum_bytes(
            &self.to_bytes()?,
            Self::check_checksum_num_bytes(Self::get_checksum_num_bytes())?,
            Self::get_check_summable_type(),
        )
    }

    fn append_checksum_bytes(&self) -> Result<Bytes> {
        let mut return_value = self.to_bytes()?;
        self.get_checksum_bytes()?
            .iter()
            .for_each(|byte| return_value.push(*byte));
        Ok(return_value)
    }
}
