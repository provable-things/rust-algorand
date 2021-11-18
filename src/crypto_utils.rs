use base32::{encode as encodeInBase32, Alphabet as Base32Alphabet};
use sha2::{Digest, Sha512Trunc256};

use crate::types::{Byte, Bytes};

pub fn sha512_256_hash_bytes(bytes: &[u8]) -> Bytes {
    let mut hasher = Sha512Trunc256::new();
    hasher.update(bytes);
    hasher.finalize().to_vec()
}

pub fn base32_encode(bytes: &[Byte]) -> String {
    encodeInBase32(Base32Alphabet::RFC4648 { padding: true }, bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_sha512_256_hash_bytes_correctly() {
        let result = hex::encode(&sha512_256_hash_bytes(b""));
        // NOTE: `expected_result from https://en.wikipedia.org/wiki/SHA-2
        let expected_result = "c672b8d1ef56ed28ab87c3622c5114069bdd3ad7b8f9737498d0c01ecef0967a";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_base32_encode_correctly() {
        // NOTE: `expected_result` from: https://www.npmjs.com/package/hi-base32
        let expected_result = "JVQW4IDJOMQGI2LTORUW4Z3VNFZWQZLEFQQG433UEBXW43DZEBRHSIDINFZSA4TFMFZW63RMEBRHK5BAMJ4SA5DINFZSA43JNZTXK3DBOIQHAYLTONUW63RAMZZG63JAN52GQZLSEBQW42LNMFWHGLBAO5UGSY3IEBUXGIDBEBWHK43UEBXWMIDUNBSSA3LJNZSCYIDUNBQXIIDCPEQGCIDQMVZHGZLWMVZGC3TDMUQG6ZRAMRSWY2LHNB2CA2LOEB2GQZJAMNXW45DJNZ2WKZBAMFXGIIDJNZSGKZTBORUWOYLCNRSSAZ3FNZSXEYLUNFXW4IDPMYQGW3TPO5WGKZDHMUWCAZLYMNSWKZDTEB2GQZJAONUG64TUEB3GK2DFNVSW4Y3FEBXWMIDBNZ4SAY3BOJXGC3BAOBWGKYLTOVZGKLQ=";
        let result = base32_encode(b"Man is distinguished, not only by his reason, but by this singular passion from other animals, which is a lust of the mind, that by a perseverance of delight in the continued and indefatigable generation of knowledge, exceeds the short vehemence of any carnal pleasure.");
        assert_eq!(result, expected_result);
    }
}
