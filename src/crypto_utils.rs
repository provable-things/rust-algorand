use sha2::{Digest, Sha512Trunc256};

use crate::types::Bytes;

pub fn sha512_256_hash_bytes(bytes: &[u8]) -> Bytes {
    let mut hasher = Sha512Trunc256::new();
    hasher.update(bytes);
    hasher.finalize().to_vec()
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
}
