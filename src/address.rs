use base32::{encode as encodeInBase32, Alphabet as Base32Alphabet};
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, SECRET_KEY_LENGTH};
use rand::rngs::OsRng;

use crate::{
    crypto_utils::{base32_encode, sha512_256_hash_bytes},
    types::{Bytes, Result},
};

const ALGORAND_CHECKSUM_LENGTH: usize = 4;
const ALGORAND_ADDRESS_LENGTH: usize = 58;

#[derive(Debug)]
pub struct Address {
    // TODO rename
    keypair: Keypair,
}

impl Address {
    pub fn create_random() -> Self {
        Self {
            keypair: Keypair::generate(&mut OsRng {}),
        }
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let secret_key = SecretKey::from_bytes(bytes)?;
        let public_key: PublicKey = (&secret_key).into();
        Ok(Self {
            keypair: Keypair {
                secret: secret_key,
                public: public_key,
            },
        })
    }

    fn to_bytes(&self) -> Bytes {
        self.keypair.secret.to_bytes().to_vec()
    }

    fn to_pub_key_bytes(&self) -> [u8; SECRET_KEY_LENGTH] {
        self.keypair.public.to_bytes()
    }

    fn compute_checksum(&self) -> Bytes {
        sha512_256_hash_bytes(&self.to_pub_key_bytes())[SECRET_KEY_LENGTH - ALGORAND_CHECKSUM_LENGTH..].to_vec()
    }

    fn to_address(&self) -> String {
        let checksum = self.compute_checksum();
        let bytes = self.to_pub_key_bytes();
        let concatted = [bytes.to_vec(), checksum].concat();
        let result = base32_encode(&concatted);
        result[0..ALGORAND_ADDRESS_LENGTH].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_private_key_bytes() -> Bytes {
        hex::decode("39564e488e19cdaf66684e06e285afa18ea3cb9f6e9e129d2d97379002b5f86e").unwrap()
    }

    fn get_sample_address() -> Address {
        Address::from_bytes(&get_sample_private_key_bytes()).unwrap()
    }

    #[test]
    fn should_create_random_address() {
        Address::create_random();
    }

    #[test]
    fn should_get_key_pair_from_bytes() {
        let bytes = get_sample_private_key_bytes();
        assert_eq!(bytes.len(), SECRET_KEY_LENGTH);
        let result = Address::from_bytes(&bytes);
        assert!(result.is_ok());
    }

    #[test]
    fn should_fail_to_get_address_from_wrong_number_of_bytes() {
        let bytes = hex::decode("39564e488e19cdaf66684e06e285afa18ea3cb9f6e9e129d2d97379002b5f8").unwrap();
        assert_ne!(bytes.len(), SECRET_KEY_LENGTH);
        let result = Address::from_bytes(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn should_make_bytes_serde_roundtrip() {
        let address_1 = Address::create_random();
        let expected_result = address_1.to_bytes();
        let address_2 = Address::from_bytes(&expected_result).unwrap();
        let result = address_2.to_bytes();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_compute_checksum() {
        let address = get_sample_address();
        let result = hex::encode(address.compute_checksum());
        let expectedResult = "d521cffd";
        assert_eq!(result, expectedResult);
    }

    #[test]
    fn should_get_address() {
        let address = get_sample_address();
        let result = address.to_address();
        let expected_result = "SCBGSYG3BCPOKY3CMZQA2VVJ6QPV2A36LSIKDAAH4OCPYFKYMA65KIOP7U";
        assert_eq!(result, expected_result);
    }
}
