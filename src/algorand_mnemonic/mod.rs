use std::fmt;

mod english_bip39_wordlist;

use crate::{
    algorand_mnemonic::english_bip39_wordlist::{
        ENGLISH_BIP_39_WORDS_HASH_MAP,
        ENGLISH_BIP_39_WORD_LIST,
    },
    algorand_types::{Byte, Bytes, Result},
    crypto_utils::sha512_256_hash_bytes,
};

const BITS_IN_A_BYTE: usize = 8;
const NUMBER_OF_BITS_PER_WORD: usize = 11;
const NUMBER_OF_WORDS_IN_MNEMONIC: usize = 25;
const NUMBER_OF_BYTES_IN_PRIVATE_KEY: usize = 32;
const NUMBER_OF_WORDS_IN_BIP_39_WORDLIST: usize = 2048;

/// ## Algorand Mnemonic
///
/// A human readable form of the Alogrand private key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlgorandMnemonic(String);

impl fmt::Display for AlgorandMnemonic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AlgorandMnemonic {
    /// ## To Bytes
    ///
    /// Converts an AlgorandMnemonic to the entropy used to create it.
    pub fn to_bytes(&self) -> Result<Bytes> {
        let words = self.safely_to_words()?;
        let bytes = Self::safely_get_indices_from_words(words.clone())
            .map(|ref indices| Self::convert_u11_array_to_bytes(indices))?;
        let checksum_word =
            Self::get_checksum_word_from_bytes(&bytes[..NUMBER_OF_BYTES_IN_PRIVATE_KEY])?;
        if checksum_word == words[words.len() - 1] {
            Ok(bytes[..NUMBER_OF_BYTES_IN_PRIVATE_KEY].to_vec())
        } else {
            Err("Invalid checksum!".into())
        }
    }

    /// ## From Bytes
    ///
    /// Converts bytes to an AlgorandMnemonic.
    pub fn from_bytes(bytes: &[Byte]) -> Result<Self> {
        Self::check_number_of_bytes(bytes)
            .map(Self::convert_bytes_to_u11_array)
            .and_then(Self::convert_u11_array_to_words)
            .and_then(|mut words| {
                words.push(Self::get_checksum_word_from_bytes(bytes)?);
                Ok(words)
            })
            .map(Self::convert_words_to_mnemonic)
    }

    /// ## From Str
    ///
    /// Converts a str to an AlgorandMnemonic.
    pub fn from_str(s: &str) -> Result<Self> {
        Self::check_number_of_words(Self::str_to_words(s)).and_then(|words| {
            match Self::safely_get_indices_from_words(words) {
                Ok(_) => Ok(Self(s.to_string())),
                Err(e) => Err(e),
            }
        })
    }

    fn check_number_of_bytes(bytes: &[Byte]) -> Result<&[Byte]> {
        let number_of_bytes = bytes.len();
        if number_of_bytes == NUMBER_OF_BYTES_IN_PRIVATE_KEY {
            Ok(bytes)
        } else {
            Err(format!(
                "Algroand mnemonic requires {} bytes, found {} bytes!",
                NUMBER_OF_BYTES_IN_PRIVATE_KEY, number_of_bytes
            )
            .into())
        }
    }

    fn convert_u11_array_to_bytes(u11_bit_number_array: &[u32]) -> Vec<u8> {
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

    fn convert_bytes_to_u11_array(bytes: &[Byte]) -> Vec<u32> {
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

    fn convert_u11_array_to_words<'a>(u11_array: Vec<u32>) -> Result<Vec<&'a str>> {
        u11_array
            .iter()
            .map(|u11| *u11 as usize)
            .map(Self::safely_get_word_from_list)
            .collect()
    }

    fn get_checksum_word_from_bytes(bytes: &[Byte]) -> Result<&str> {
        // TODO test!
        Self::convert_u11_array_to_words(Self::convert_bytes_to_u11_array(
            &sha512_256_hash_bytes(bytes)[..2],
        ))
        .and_then(|words| {
            if words.is_empty() {
                Err("Error getting checksum word from bytes!".into())
            } else {
                Ok(words[0])
            }
        })
    }

    fn convert_words_to_mnemonic(words: Vec<&str>) -> Self {
        let mut reversed_words = words.clone();
        reversed_words.reverse();
        Self(
            reversed_words
                .iter()
                .enumerate()
                .fold(String::new(), |mnemonic, (i, word)| {
                    if i == 0 {
                        word.to_string()
                    } else {
                        format!("{} {}", word, mnemonic)
                    }
                }),
        )
    }

    fn safely_get_word_from_list<'a>(index: usize) -> Result<&'a str> {
        match ENGLISH_BIP_39_WORDS_HASH_MAP.get(&index) {
            Some(word) => Ok(word),
            None => Err(format!(
                "Cannot get word number {}! BIP39 word list is only {} words long!",
                index, NUMBER_OF_WORDS_IN_BIP_39_WORDLIST
            )
            .into()),
        }
    }

    fn safely_get_index_from_word(word: &str) -> Result<u32> {
        ENGLISH_BIP_39_WORD_LIST
            .iter()
            .position(|bip_39_word| *bip_39_word == word)
            .ok_or_else(|| format!("No '{}' in english BIP39 word list!", word).into())
            .map(|u_size| u_size as u32)
    }

    fn safely_get_indices_from_words(words: Vec<&str>) -> Result<Vec<u32>> {
        words
            .iter()
            .map(|word| Self::safely_get_index_from_word(word))
            .collect()
    }

    fn safely_to_words(&self) -> Result<Vec<&str>> {
        Self::check_number_of_words(Self::str_to_words(&self.0))
    }

    fn str_to_words(s: &str) -> Vec<&str> {
        s.split(' ').collect()
    }

    fn check_number_of_words(words: Vec<&str>) -> Result<Vec<&str>> {
        // TODO test!
        let number_of_words = words.len();
        if number_of_words != NUMBER_OF_WORDS_IN_MNEMONIC {
            Err(format!(
                "Expected {} words in mnemonic, but got {} instead!",
                NUMBER_OF_WORDS_IN_MNEMONIC, number_of_words
            )
            .into())
        } else {
            Ok(words)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::AppError;

    fn get_sample_mnemonic() -> AlgorandMnemonic {
        AlgorandMnemonic::convert_words_to_mnemonic(get_sample_words())
    }

    fn get_sample_words<'a>() -> Vec<&'a str> {
        AlgorandMnemonic::str_to_words(get_sample_mnemonic_str())
    }

    fn get_sample_mnemonic_str() -> &'static str {
        "shrimp deer category ocean olive program drip example dolphin bleak style tube either very insane oyster pelican reopen slide address ahead coil jelly about gossip"
    }

    fn get_sample_words_without_checksum<'a>() -> Vec<&'a str> {
        get_sample_words()[..NUMBER_OF_WORDS_IN_MNEMONIC - 1].to_vec()
    }

    fn get_sample_private_key_bytes() -> Bytes {
        hex::decode("39564e488e19cdaf66684e06e285afa18ea3cb9f6e9e129d2d97379002b5f86e").unwrap()
    }

    #[test]
    fn should_convert_bytes_to_u11_array() {
        let expected_result = vec![
            1593, 458, 289, 1223, 1233, 1375, 537, 627, 518, 188, 1726, 1872, 568, 1943, 935, 1267,
            1298, 1459, 1628, 27, 41, 362, 958, 3,
        ];
        let result = AlgorandMnemonic::convert_bytes_to_u11_array(&get_sample_private_key_bytes());
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_convert_u11_array_to_words() {
        let u11_array =
            AlgorandMnemonic::convert_bytes_to_u11_array(&get_sample_private_key_bytes());
        let result = AlgorandMnemonic::convert_u11_array_to_words(u11_array).unwrap();
        assert_eq!(result, get_sample_words_without_checksum());
    }

    #[test]
    fn should_safely_get_word_from_list() {
        let index = 1337;
        let expected_result = "poet";
        let result = AlgorandMnemonic::safely_get_word_from_list(index).unwrap();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_err_when_safely_getting_out_of_range_word_from_list() {
        let index = NUMBER_OF_WORDS_IN_BIP_39_WORDLIST + 1;
        let expected_error = format!(
            "Cannot get word number {}! BIP39 word list is only {} words long!",
            index, NUMBER_OF_WORDS_IN_BIP_39_WORDLIST
        );
        match AlgorandMnemonic::safely_get_word_from_list(index) {
            Ok(_) => panic!("Should not have succeeded!"),
            Err(AppError::Custom(error)) => assert_eq!(error, expected_error),
            Err(_) => panic!("Wrong error received!"),
        }
    }

    #[test]
    fn should_convert_u11_array_to_bytes() {
        let u11_array =
            AlgorandMnemonic::convert_bytes_to_u11_array(&get_sample_private_key_bytes());
        let result = AlgorandMnemonic::convert_u11_array_to_bytes(&u11_array);
        let expected_result = vec![
            57, 86, 78, 72, 142, 25, 205, 175, 102, 104, 78, 6, 226, 133, 175, 161, 142, 163, 203,
            159, 110, 158, 18, 157, 45, 151, 55, 144, 2, 181, 248, 110,
        ];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_convert_between_u8_and_u11_arrays_successfully() {
        let u11_array =
            AlgorandMnemonic::convert_bytes_to_u11_array(&get_sample_private_key_bytes());
        let u8_array = AlgorandMnemonic::convert_u11_array_to_bytes(&u11_array);
        let result = AlgorandMnemonic::convert_bytes_to_u11_array(&u8_array);
        assert_eq!(result, u11_array);
    }

    #[test]
    fn should_convert_bytes_to_mnemonic() {
        let bytes = get_sample_private_key_bytes();
        let result = AlgorandMnemonic::from_bytes(&bytes).unwrap();
        assert_eq!(result, get_sample_mnemonic());
    }

    #[test]
    fn should_convert_words_to_mnemonic() {
        let words = get_sample_words();
        let result = AlgorandMnemonic::convert_words_to_mnemonic(words);
        assert_eq!(result, get_sample_mnemonic());
    }

    #[test]
    fn should_convert_mnemonic_to_words() {
        let mnemonic = get_sample_mnemonic();
        let result = AlgorandMnemonic::safely_to_words(&mnemonic).unwrap();
        assert_eq!(result, get_sample_words());
    }

    #[test]
    fn should_fail_to_convert_mnemonic_to_words() {
        let short_mnemonic = AlgorandMnemonic("not enough words".to_string());
        let expected_error = format!(
            "Expected {} words in mnemonic, but got {} instead!",
            NUMBER_OF_WORDS_IN_MNEMONIC, 3,
        );
        match AlgorandMnemonic::safely_to_words(&short_mnemonic) {
            Ok(_) => panic!("Should not have succeeded!"),
            Err(AppError::Custom(error)) => assert_eq!(error, expected_error),
            Err(_) => panic!("Wrong error received!"),
        }
    }

    #[test]
    fn should_safely_get_index_from_word() {
        let word = "poet";
        let expected_result = 1337;
        let result = AlgorandMnemonic::safely_get_index_from_word(word).unwrap();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_fail_safely_get_index_from_word() {
        let word = "notinlist";
        let expected_error = format!("No '{}' in english BIP39 word list!", word);
        match AlgorandMnemonic::safely_get_index_from_word(word) {
            Ok(_) => panic!("Should not have succeeded!"),
            Err(AppError::Custom(error)) => assert_eq!(error, expected_error),
            Err(_) => panic!("Wrong error received!"),
        }
    }

    #[test]
    fn should_convert_mnemonic_to_bytes() {
        let result = AlgorandMnemonic::to_bytes(&get_sample_mnemonic()).unwrap();
        assert_eq!(result, get_sample_private_key_bytes());
    }

    #[test]
    fn should_make_mnemoic_to_bytes_roundtrip() {
        let bytes = get_sample_private_key_bytes();
        let mnemonic = AlgorandMnemonic::from_bytes(&bytes).unwrap();
        let result = AlgorandMnemonic::to_bytes(&mnemonic).unwrap();
        assert_eq!(result, bytes);
    }

    #[test]
    fn should_get_algorand_mnemonic_from_str() {
        let mnemonic_str = get_sample_mnemonic_str();
        let result = AlgorandMnemonic::from_str(mnemonic_str);
        assert!(result.is_ok());
    }

    #[test]
    fn should_get_words_from_str() {
        let word_1 = "word1";
        let word_2 = "word2";
        let word_3 = "word3";
        let s = format!("{} {} {}", word_1, word_2, word_3);
        let expected_result = vec![word_1, word_2, word_3];
        let result = AlgorandMnemonic::str_to_words(&s);
        assert_eq!(result, expected_result)
    }

    #[test]
    fn should_check_number_of_words() {
        let words = get_sample_words();
        let result = AlgorandMnemonic::check_number_of_words(words);
        assert!(result.is_ok());
    }

    #[test]
    fn should_fail_check_on_number_of_words_if_number_incorrect() {
        let incorrect_words = vec!["not", "the", "correct", "amount", "of", "words"];
        let expected_error = format!(
            "Expected {} words in mnemonic, but got {} instead!",
            NUMBER_OF_WORDS_IN_MNEMONIC,
            incorrect_words.len()
        );
        match AlgorandMnemonic::check_number_of_words(incorrect_words) {
            Ok(_) => panic!("Should not have succeeded!"),
            Err(AppError::Custom(error)) => assert_eq!(error, expected_error),
            Err(_) => panic!("Wrong error received!"),
        }
    }
}
