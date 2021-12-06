use base64::decode as base64_decode;
use derive_more::Constructor;
use ed25519_dalek::{
    Keypair,
    PublicKey,
    SecretKey,
    Signature as DalekSignature,
    Signer,
    PUBLIC_KEY_LENGTH,
    SECRET_KEY_LENGTH,
};
use rand::rngs::OsRng;
use serde::Serialize;

use crate::{
    algorand_address::{
        AlgorandAddress,
        ALGORAND_ADDRESS_BASE_32_NUM_BYTES,
        ALGORAND_CHECKSUM_NUM_BYTES,
    },
    algorand_signature::AlgorandSignature,
    crypto_utils::{base32_encode_with_padding, sha512_256_hash_bytes},
    mnemonic::AlgorandMnemonic,
    types::{Byte, Bytes, Result},
};

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
            [PUBLIC_KEY_LENGTH - ALGORAND_CHECKSUM_NUM_BYTES..]
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
    pub fn to_address(&self) -> Result<AlgorandAddress> {
        AlgorandAddress::from_bytes(
            &[self.to_pub_key_bytes().to_vec(), self.compute_checksum()].concat(),
        )
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
    pub fn to_mnemonic(&self) -> Result<AlgorandMnemonic> {
        AlgorandMnemonic::from_bytes(&self.to_bytes())
    }

    /// ## From Mnemonic
    ///
    /// Get the algorand keys from a mnemonic.
    pub fn from_mnemonic(mnemonic: &AlgorandMnemonic) -> Result<Self> {
        mnemonic
            .to_bytes()
            .and_then(|ref bytes| Self::from_bytes(bytes))
    }

    /// ## Sign
    ///
    /// Sign the passed in message bytes with the private key.
    pub fn sign(&self, message: &[Byte]) -> AlgorandSignature {
        AlgorandSignature::from_byte_array(self.0.sign(message).to_bytes())
    }

    /// ## Verify
    ///
    /// Verify the passed in message & signature were signed by this keypair.
    pub fn verify(&self, message: &[Byte], signature: &AlgorandSignature) -> Result<()> {
        Ok(self
            .0
            .verify(message, &DalekSignature::new(signature.to_byte_array()))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        get_sample_address,
        get_sample_algorand_keys,
        get_sample_mnemonic,
        get_sample_private_key_bytes,
    };

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
        let result = address.to_address().unwrap().to_string();
        let expected_result = "SCBGSYG3BCPOKY3CMZQA2VVJ6QPV2A36LSIKDAAH4OCPYFKYMA65KIOP7U";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_decode_base_64_encoded_secret() {
        let base64_encoded_secret = "IEirzzmZ3mDcl/qj25Ffo71s/dDvFxIGS2H89LaViFbn8PhNBoEd+fMcjYeLEVX0Zx1RoYXCAJCGZ/RJWHBooQ==";
        let keys = AlgorandKeys::from_base_64_encoded_secret(base64_encoded_secret).unwrap();
        let result = keys.to_address().unwrap().to_string();
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

    #[test]
    fn should_get_alogrand_keys_from_mnemonic() {
        let mnemonic = get_sample_mnemonic();
        let keys = AlgorandKeys::from_mnemonic(&mnemonic).unwrap();
        let result = keys.to_bytes();
        let expected_result = get_sample_private_key_bytes();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_convert_mnemonic_to_address() {
        let mnemonic_str = "income valve harsh cat anger online hole quality economy tiny alarm pipe great forget language cereal swear humble rely desk sell palm zebra abstract grab";
        let mnemonic = AlgorandMnemonic::from_str(mnemonic_str).unwrap();
        let expected_result = "GKDMGXNL44BCEQ4M7HUBPKPY3H5O6DMI7YG36GD2WZU2MPFWMVY4RWG3FE";
        let result = AlgorandKeys::from_mnemonic(&mnemonic)
            .unwrap()
            .to_address()
            .unwrap()
            .to_string();
        assert_eq!(result, expected_result);
    }

    fn get_expected_signature_hex() -> String {
        "2abcdf146c0c222b7955181fde447c5818a28fd69c3d88e487ef8e8dfc1bd4319dd8a810d9bfbdb52c38c9346e57801e8d0bef6968eaac7c3913ad51ee21c00e".to_string()
    }

    fn get_message_to_sign() -> Bytes {
        b"some message".to_vec()
    }

    #[test]
    fn should_sign_message() {
        let keys = get_sample_algorand_keys();
        let message = get_message_to_sign();
        let result = hex::encode(keys.sign(&message).to_byte_array());
        let expected_result = get_expected_signature_hex();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_verify_message_signature() {
        let keys = get_sample_algorand_keys();
        let message = get_message_to_sign();
        let signature = keys.sign(&message);
        let result = keys.verify(&message, &signature);
        assert!(result.is_ok());
    }

    #[test]
    fn should_error_if_signature_is_not_valid() {
        let keys_1 = AlgorandKeys::create_random();
        let keys_2 = get_sample_algorand_keys();
        assert_ne!(keys_1.to_address().unwrap(), keys_2.to_address().unwrap());
        let message = get_message_to_sign();
        let signature = keys_1.sign(&message);
        let result = keys_2.verify(&message, &signature);
        assert!(result.is_err());
    }
}
