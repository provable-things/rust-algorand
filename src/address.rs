use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature};
use rand::rngs::OsRng;

use crate::types::{Bytes, Result};

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
}

#[cfg(test)]
mod tests {
    use ed25519_dalek::SECRET_KEY_LENGTH;

    use super::*;

    #[test]
    fn should_create_random_address() {
        Address::create_random();
    }

    #[test]
    fn should_get_key_pair_from_bytes() {
        let bytes = hex::decode("39564e488e19cdaf66684e06e285afa18ea3cb9f6e9e129d2d97379002b5f86e").unwrap();
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
}
