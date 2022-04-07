use std::{fmt::Display, str::FromStr};

use base64::{decode as base64_decode, encode as base64_encode};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    algorand_errors::AlgorandError,
    algorand_hash::AlgorandHash,
    algorand_types::{Bytes, Result},
};

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
struct AlgorandProofJson {
    proof: String,

    #[serde(rename = "idx")]
    index: u64,

    #[serde(rename = "stibhash")]
    stib_hash: String,

    #[serde(rename = "hashtype")]
    hash_type: String,

    #[serde(rename = "treedepth")]
    tree_depth: u64,
}

impl Display for AlgorandProofJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", json!(self).to_string())
    }
}

impl FromStr for AlgorandProofJson {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct AlgorandProof {
    index: u64,
    tree_depth: u64,
    stib_hash: Bytes,
    hash_type: String,
    proof: Vec<Bytes>,
}

impl FromStr for AlgorandProof {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_json(&AlgorandProofJson::from_str(s)?)
    }
}

impl Display for AlgorandProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_json())
    }
}

impl AlgorandProof {
    fn decode_proof_str(s: &str) -> Result<Vec<Bytes>> {
        let bytes = base64_decode(s)?;
        let result = bytes
            .iter()
            .fold(
                (Vec::new(), Vec::new()),
                |(mut individual_hashes, mut bytes), byte| {
                    bytes.push(byte.clone());
                    if bytes.len() == 32 {
                        individual_hashes.push(bytes);
                        bytes = vec![];
                    };
                    (individual_hashes, bytes)
                },
            )
            .0;
        Ok(result)
    }

    pub fn from_json(json: &AlgorandProofJson) -> Result<Self> {
        Ok(Self {
            hash_type: json.hash_type.clone(),
            index: json.index,
            tree_depth: json.tree_depth,
            stib_hash: base64_decode(&json.stib_hash)?,
            proof: Self::decode_proof_str(&json.proof)?,
        })
    }

    pub fn to_json(&self) -> AlgorandProofJson {
        AlgorandProofJson {
            index: self.index,
            tree_depth: self.tree_depth,
            hash_type: self.hash_type.clone(),
            stib_hash: base64_encode(&self.stib_hash),
            proof: base64_encode(&self.proof.concat()),
        }
    }
}

#[cfg(test)]
mod tests {
    use base64::{decode as base64_decode, encode as base64_encode};

    use super::*;
    use crate::{
        algorand_hash::AlgorandHash,
        algorand_types::Bytes,
        crypto_utils::{base32_decode, base32_encode_with_no_padding, sha512_256_hash_bytes},
    };

    fn get_sample_proof_string() -> String {
        json!({
            "hashtype": "sha512_256",
            "idx": 47,
            "proof": "RqFhu2v3tWDNzQYYBqIIogwlVNfouGZHL8SysDYZFyAyF3jY3e/Of399c18S0nZT7ggITIM2xF3H+Z7HNA+4uVmR5/2f9ev6j9xOTKDxM4F5ObtyQPNIEZiwa866kGUCabEFj8JyXjJ0oYvnVrmjXTaSwXouDFoh4lGkExkhu+wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH1Mjdbvd94NJ7I2ysmkH/LoNr+Jk8S4WjIcZMf4HmRF",
            "stibhash": "p1VyFS6idjmUxZpYusk96vrbfYWDXgv127inV6kDBlo=",
            "treedepth": 6
        }).to_string()
    }

    fn get_sample_proof_json() -> AlgorandProofJson {
        AlgorandProofJson::from_str(&get_sample_proof_string()).unwrap()
    }

    fn get_sample_proof() -> AlgorandProof {
        AlgorandProof::from_str(&get_sample_proof_string()).unwrap()
    }

    #[test]
    fn should_serde_proof_to_and_from_str() {
        let proof = get_sample_proof();
        let s = proof.to_string();
        let result = AlgorandProof::from_str(&s).unwrap();
        assert_eq!(result, proof);
    }
}
