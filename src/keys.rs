use base64::decode as base64_decode;
use ed25519_dalek::{Keypair, PublicKey, SecretKey, SECRET_KEY_LENGTH};
use rand::rngs::OsRng;

use crate::{
    crypto_utils::{base32_encode, sha512_256_hash_bytes},
    mnemonic::{convert_bytes_to_mnemonic, convert_mnemonic_to_bytes},
    types::{Bytes, Result},
};

const ALGORAND_CHECKSUM_LENGTH: usize = 4;
const ALGORAND_ADDRESS_LENGTH: usize = 58;

#[derive(Debug)]
pub struct AlgorandKeys(Keypair);

/// ## AlgorandKeys
///
/// A struct holding a public and private asymmetric key pair derived from the ed25519 curve.
impl AlgorandKeys {
    fn to_pub_key_bytes(&self) -> [u8; SECRET_KEY_LENGTH] {
        self.0.public.to_bytes()
    }

    fn compute_checksum(&self) -> Bytes {
        sha512_256_hash_bytes(&self.to_pub_key_bytes())
            [SECRET_KEY_LENGTH - ALGORAND_CHECKSUM_LENGTH..]
            .to_vec()
    }

    /// ## Create Random
    ///
    /// Generates a random keypair using entropy from the operating system.
    pub fn create_random() -> Self {
        Self(Keypair::generate(&mut OsRng {}))
    }

    /// ## From Bytes
    ///
    /// Create the algorand key pair from the 32 bytes of a private key.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let secret_key = SecretKey::from_bytes(bytes)?;
        let public_key: PublicKey = (&secret_key).into();
        Ok(Self(Keypair {
            secret: secret_key,
            public: public_key,
        }))
    }

    /// ## To Bytes
    ///
    /// Convert the private key to bytes.
    pub fn to_bytes(&self) -> Bytes {
        self.0.secret.to_bytes().to_vec()
    }

    /// ## To Address
    ///
    /// Convert the algorand keypair to an algorand address.
    pub fn to_address(&self) -> String {
        base32_encode(&[self.to_pub_key_bytes().to_vec(), self.compute_checksum()].concat())
            [0..ALGORAND_ADDRESS_LENGTH]
            .to_string()
    }

    /// ## To Address
    ///
    /// Create the keypair from a base64 encoded key pair.
    pub fn from_base_64_encoded_secret(s: &str) -> Result<Self> {
        Self::from_bytes(&base64_decode(s)?[..SECRET_KEY_LENGTH])
    }

    /// ## To Mnemonic
    ///
    /// Output the private key as a human-readable mnemonic.
    pub fn to_mnemonic(&self) -> Result<String> {
        convert_bytes_to_mnemonic(&self.to_bytes())
    }

    /// ## From Mnemonic
    ///
    /// Get the algorand keys from a mnemonic.
    pub fn from_mnemonic(mnemonic: &str) -> Result<Self> {
        convert_mnemonic_to_bytes(mnemonic).and_then(|ref bytes| Self::from_bytes(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_private_key_bytes() -> Bytes {
        hex::decode("39564e488e19cdaf66684e06e285afa18ea3cb9f6e9e129d2d97379002b5f86e").unwrap()
    }

    fn get_sample_algorand_keys() -> AlgorandKeys {
        AlgorandKeys::from_bytes(&get_sample_private_key_bytes()).unwrap()
    }

    fn get_sample_address() -> AlgorandKeys {
        AlgorandKeys::from_bytes(&get_sample_private_key_bytes()).unwrap()
    }

    #[test]
    fn should_create_random_keys() {
        AlgorandKeys::create_random();
    }

    #[test]
    fn should_get_keys_from_bytes() {
        let bytes = get_sample_private_key_bytes();
        assert_eq!(bytes.len(), SECRET_KEY_LENGTH);
        let result = AlgorandKeys::from_bytes(&bytes);
        assert!(result.is_ok());
    }

    #[test]
    fn should_fail_to_get_keys_from_wrong_number_of_bytes() {
        let bad_bytes =
            hex::decode("39564e488e19cdaf66684e06e285afa18ea3cb9f6e9e129d2d97379002b5f8").unwrap();
        assert_ne!(bad_bytes.len(), SECRET_KEY_LENGTH);
        let result = AlgorandKeys::from_bytes(&bad_bytes);
        assert!(result.is_err());
    }

    #[test]
    fn keys_should_make_bytes_serde_roundtrip() {
        let address_1 = AlgorandKeys::create_random();
        let expected_result = address_1.to_bytes();
        let address_2 = AlgorandKeys::from_bytes(&expected_result).unwrap();
        let result = address_2.to_bytes();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_compute_checksum_of_keys() {
        let address = get_sample_address();
        let result = hex::encode(address.compute_checksum());
        let expected_result = "d521cffd";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_get_address_from_keys() {
        let address = get_sample_address();
        let result = address.to_address();
        let expected_result = "SCBGSYG3BCPOKY3CMZQA2VVJ6QPV2A36LSIKDAAH4OCPYFKYMA65KIOP7U";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_decode_base_64_encoded_secret() {
        let base64_encoded_secret = "IEirzzmZ3mDcl/qj25Ffo71s/dDvFxIGS2H89LaViFbn8PhNBoEd+fMcjYeLEVX0Zx1RoYXCAJCGZ/RJWHBooQ==";
        let keys = AlgorandKeys::from_base_64_encoded_secret(base64_encoded_secret).unwrap();
        let result = keys.to_address();
        // NOTE: Sample taken from js-algorand-sdk
        let expected_result = "47YPQTIGQEO7T4Y4RWDYWEKV6RTR2UNBQXBABEEGM72ESWDQNCQ52OPASU";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_convert_key_to_mnemonic() {
        let key = get_sample_algorand_keys();
        let result = key.to_mnemonic().unwrap();
        let expected_result = get_sample_mnemonic();
        assert_eq!(result, expected_result);
    }

    fn get_sample_mnemonic() -> &'static str {
        "shrimp deer category ocean olive program drip example dolphin bleak style tube either very insane oyster pelican reopen slide address ahead coil jelly about gossip"
    }

    #[test]
    fn should_get_alogrand_keys_from_mnemonic() {
        let mnemonic = get_sample_mnemonic();
        let keys = AlgorandKeys::from_mnemonic(mnemonic).unwrap();
        let result = keys.to_bytes();
        let expected_result = get_sample_private_key_bytes();
        assert_eq!(result, expected_result);
    }
}
