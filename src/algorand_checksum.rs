use crate::{
    algorand_types::{Byte, Bytes},
    crypto_utils::sha512_256_hash_bytes,
};

pub type u11Array = Vec<u32>;
const BITS_IN_A_BYTE: usize = 8;
const NUMBER_OF_BITS_PER_WORD: usize = 11;
const ALGORAND_NUM_CHECKSUM_BYTES: usize = 2;

pub struct AlgorandChecksum;

impl AlgorandChecksum {
    pub fn convert_bytes_to_u11_array(bytes: &[Byte]) -> u11Array {
        const ELEVEN_BITS_MASK: u32 = 0x7ffu32;
        let mut buffer = 0u32;
        let mut bit_count = 0;
        let mut result = Vec::new();
        bytes.iter().for_each(|byte| {
            buffer |= (u32::from(*byte)) << bit_count;
            bit_count += BITS_IN_A_BYTE;
            if bit_count >= NUMBER_OF_BITS_PER_WORD {
                result.push(buffer & ELEVEN_BITS_MASK);
                buffer >>= NUMBER_OF_BITS_PER_WORD as u32;
                bit_count -= NUMBER_OF_BITS_PER_WORD;
            }
        });
        if bit_count != 0 {
            result.push(buffer & ELEVEN_BITS_MASK);
        }
        result
    }

    pub fn convert_u11_array_to_bytes(u11_bit_number_array: &[u32]) -> Bytes {
        const EIGHT_BITS_MASK: u32 = 0xff;
        let mut buffer = 0;
        let mut bit_count = 0;
        let mut result = Vec::new();
        for &u11_bit_number in u11_bit_number_array {
            buffer |= u11_bit_number << bit_count;
            bit_count += NUMBER_OF_BITS_PER_WORD as u32;
            while bit_count >= 8 {
                result.push((buffer & EIGHT_BITS_MASK) as u8);
                buffer >>= 8;
                bit_count -= 8;
            }
        }
        if bit_count != 0 {
            result.push((buffer & EIGHT_BITS_MASK) as u8)
        }
        result[..32].to_vec()
    }

    pub fn get_checksum_bytes(bytes: &[Byte]) -> Bytes {
        sha512_256_hash_bytes(bytes)[..ALGORAND_NUM_CHECKSUM_BYTES].to_vec()
    }

    /*
    pub fn append_checksum_bytes(bytes: &[Byte]) -> Bytes {
        Self::get_checksum_bytes(bytes)
    }
    */
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorand_types::Bytes;

    fn get_sample_bytes() -> Bytes {
        hex::decode("39564e488e19cdaf66684e06e285afa18ea3cb9f6e9e129d2d97379002b5f86e").unwrap()
    }

    #[test]
    fn should_convert_bytes_to_u11_array() {
        let expected_result = vec![
            1593, 458, 289, 1223, 1233, 1375, 537, 627, 518, 188, 1726, 1872, 568, 1943, 935, 1267,
            1298, 1459, 1628, 27, 41, 362, 958, 3,
        ];
        let result = AlgorandChecksum::convert_bytes_to_u11_array(&get_sample_bytes());
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_convert_u11_array_to_bytes() {
        let u11_array = AlgorandChecksum::convert_bytes_to_u11_array(&get_sample_bytes());
        let result = AlgorandChecksum::convert_u11_array_to_bytes(&u11_array);
        let expected_result = vec![
            57, 86, 78, 72, 142, 25, 205, 175, 102, 104, 78, 6, 226, 133, 175, 161, 142, 163, 203,
            159, 110, 158, 18, 157, 45, 151, 55, 144, 2, 181, 248, 110,
        ];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_convert_between_u8_and_u11_arrays_successfully() {
        let u11_array = AlgorandChecksum::convert_bytes_to_u11_array(&get_sample_bytes());
        let u8_array = AlgorandChecksum::convert_u11_array_to_bytes(&u11_array);
        let result = AlgorandChecksum::convert_bytes_to_u11_array(&u8_array);
        assert_eq!(result, u11_array);
    }

    #[test]
    fn should_get_checksum_bytes() {
        let result = AlgorandChecksum::get_checksum_bytes(&get_sample_bytes());
        let expected_result = vec![39, 219];
        assert_eq!(result, expected_result);
    }
}
