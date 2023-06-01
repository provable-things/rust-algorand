use base32::{decode as decodeFromBase32, encode as encodeInBase32, Alphabet as Base32Alphabet};
use sha2::{Digest, Sha512_256};

use crate::algorand_types::{Byte, Bytes, Result};

pub fn sha512_256_hash_bytes(bytes: &[u8]) -> Bytes {
    let mut hasher = Sha512_256::new();
    hasher.update(bytes);
    hasher.finalize().to_vec()
}

fn base_32_encode_maybe_with_padding(bytes: &[Byte], padding: bool) -> String {
    encodeInBase32(Base32Alphabet::RFC4648 { padding }, bytes)
}

pub fn base32_encode_with_no_padding(bytes: &[Byte]) -> String {
    base_32_encode_maybe_with_padding(bytes, false)
}

pub fn base32_decode(s: &str) -> Result<Bytes> {
    match decodeFromBase32(Base32Alphabet::RFC4648 { padding: false }, s) {
        Some(bytes) => Ok(bytes),
        None => Err("Error decoding string from base32!".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_bytes_to_encode() -> Bytes {
        b"Man is distinguished, not only by his reason, but by this singular passion from other animals, which is a lust of the mind, that by a perseverance of delight in the continued and indefatigable generation of knowledge, exceeds the short vehemence of any carnal pleasure.".to_vec()
    }

    fn get_encoded_str() -> String {
        "JVQW4IDJOMQGI2LTORUW4Z3VNFZWQZLEFQQG433UEBXW43DZEBRHSIDINFZSA4TFMFZW63RMEBRHK5BAMJ4SA5DINFZSA43JNZTXK3DBOIQHAYLTONUW63RAMZZG63JAN52GQZLSEBQW42LNMFWHGLBAO5UGSY3IEBUXGIDBEBWHK43UEBXWMIDUNBSSA3LJNZSCYIDUNBQXIIDCPEQGCIDQMVZHGZLWMVZGC3TDMUQG6ZRAMRSWY2LHNB2CA2LOEB2GQZJAMNXW45DJNZ2WKZBAMFXGIIDJNZSGKZTBORUWOYLCNRSSAZ3FNZSXEYLUNFXW4IDPMYQGW3TPO5WGKZDHMUWCAZLYMNSWKZDTEB2GQZJAONUG64TUEB3GK2DFNVSW4Y3FEBXWMIDBNZ4SAY3BOJXGC3BAOBWGKYLTOVZGKLQ".to_string()
    }

    #[test]
    fn should_sha512_256_hash_bytes_correctly() {
        let result = hex::encode(sha512_256_hash_bytes(b""));
        // NOTE: `expected_result from https://en.wikipedia.org/wiki/SHA-2
        let expected_result = "c672b8d1ef56ed28ab87c3622c5114069bdd3ad7b8f9737498d0c01ecef0967a";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_base32_encode_with_no_padding_correctly() {
        // NOTE: `expected_result` from: https://www.npmjs.com/package/hi-base32
        let expected_result = get_encoded_str();
        let result = base32_encode_with_no_padding(&get_bytes_to_encode());
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_decode_from_base32() {
        let decoded = base32_decode(&get_encoded_str()).unwrap();
        assert_eq!(decoded, get_bytes_to_encode());
    }
}
