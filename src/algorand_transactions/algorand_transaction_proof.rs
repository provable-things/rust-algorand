use std::{fmt::Display, str::FromStr};

use base64::{decode as base64_decode, encode as base64_encode};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    algorand_blocks::block::AlgorandBlock,
    algorand_errors::AlgorandError,
    algorand_hash::AlgorandHash,
    algorand_types::{Bytes, Result},
    crypto_utils::sha512_256_hash_bytes,
};

// NOTE: These prefixes are used to domain-separate the various hashes used in the protocol.
const MERKLE_ARRAY_ELEMTENT_PREFIX: [u8; 2] = *b"MA";
const TRANSACTION_MERKLE_LEAF_PREFIX: [u8; 2] = *b"TL";

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct AlgorandTransactionProofJson {
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

impl Display for AlgorandTransactionProofJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", json!(self))
    }
}

impl FromStr for AlgorandTransactionProofJson {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct AlgorandTransactionProof {
    pub index: u64,
    pub tree_depth: u64,
    pub stib_hash: Bytes,
    pub hash_type: String,
    pub proof: Vec<Bytes>,
}

impl FromStr for AlgorandTransactionProof {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_json(&AlgorandTransactionProofJson::from_str(s)?)
    }
}

impl Display for AlgorandTransactionProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_json())
    }
}

impl AlgorandTransactionProof {
    fn decode_proof_str(s: &str) -> Result<Vec<Bytes>> {
        let bytes = base64_decode(s)?;
        let result = bytes
            .iter()
            .fold(
                (Vec::new(), Vec::new()),
                |(mut individual_hashes, mut bytes), byte| {
                    bytes.push(*byte);
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

    pub fn from_json(json: &AlgorandTransactionProofJson) -> Result<Self> {
        Ok(Self {
            hash_type: json.hash_type.clone(),
            index: json.index,
            tree_depth: json.tree_depth,
            stib_hash: base64_decode(&json.stib_hash)?,
            proof: Self::decode_proof_str(&json.proof)?,
        })
    }

    pub fn to_json(&self) -> AlgorandTransactionProofJson {
        AlgorandTransactionProofJson {
            index: self.index,
            tree_depth: self.tree_depth,
            hash_type: self.hash_type.clone(),
            stib_hash: base64_encode(&self.stib_hash),
            proof: base64_encode(&self.proof.concat()),
        }
    }

    fn calculate_next_leaf_index(current_index: u64) -> u64 {
        (current_index - (current_index % 2)) / 2
    }

    fn calculate_leaf_hash(&self, tx_id: &AlgorandHash) -> Bytes {
        sha512_256_hash_bytes(
            &[
                TRANSACTION_MERKLE_LEAF_PREFIX.into(),
                tx_id.to_bytes(),
                self.stib_hash.clone(),
            ]
            .concat(),
        )
    }

    fn to_root_hash(&self, tx_id: &AlgorandHash) -> Result<AlgorandHash> {
        let leaf_hash = self.calculate_leaf_hash(tx_id);
        AlgorandHash::from_slice(
            &self
                .proof
                .iter()
                .fold((leaf_hash, self.index), |(hash, index), hash_from_proof| {
                    let mut bytes_to_hash = vec![MERKLE_ARRAY_ELEMTENT_PREFIX.to_vec()];
                    if index % 2 == 0 {
                        bytes_to_hash.push(hash);
                        bytes_to_hash.push(hash_from_proof.to_vec());
                    } else {
                        bytes_to_hash.push(hash_from_proof.to_vec());
                        bytes_to_hash.push(hash);
                    };
                    let next_hash = sha512_256_hash_bytes(&bytes_to_hash.concat());
                    let next_index = Self::calculate_next_leaf_index(index);
                    (next_hash, next_index)
                })
                .0,
        )
    }

    fn is_valid(&self, tx_id: &AlgorandHash, txn_root: &AlgorandHash) -> Result<bool> {
        self.to_root_hash(tx_id)
            .map(|ref root_hash| root_hash == txn_root)
    }

    pub fn validate(&self, block: &AlgorandBlock) -> Result<()> {
        // TODO test
        if self.is_valid(
            &block
                .get_transaction_at_index(self.index as usize)?
                .get_id()?,
            &block.get_transactions_root()?,
        )? {
            Ok(())
        } else {
            Err("Invalid proof!".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use base64::decode as base64_decode;

    use super::*;
    use crate::{algorand_hash::AlgorandHash, crypto_utils::base32_decode};

    fn get_sample_proof_string() -> String {
        // NOTE: Gotten via: curl -s "https://algoexplorerapi.io/v2/blocks/20261491/transactions/UFZTMQWJ3N6LWGMMSF7EJENOQKYYUDC7A2346TR3L7AYTBRCAPZQ/proof" | jq
        json!({
            "hashtype": "sha512_256",
            "idx": 47,
            "proof": "RqFhu2v3tWDNzQYYBqIIogwlVNfouGZHL8SysDYZFyAyF3jY3e/Of399c18S0nZT7ggITIM2xF3H+Z7HNA+4uVmR5/2f9ev6j9xOTKDxM4F5ObtyQPNIEZiwa866kGUCabEFj8JyXjJ0oYvnVrmjXTaSwXouDFoh4lGkExkhu+wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH1Mjdbvd94NJ7I2ysmkH/LoNr+Jk8S4WjIcZMf4HmRF",
            "stibhash": "p1VyFS6idjmUxZpYusk96vrbfYWDXgv127inV6kDBlo=",
            "treedepth": 6
        }).to_string()
    }

    fn get_sample_proof_json() -> AlgorandTransactionProofJson {
        AlgorandTransactionProofJson::from_str(&get_sample_proof_string()).unwrap()
    }

    fn get_sample_proof() -> AlgorandTransactionProof {
        AlgorandTransactionProof::from_str(&get_sample_proof_string()).unwrap()
    }

    #[test]
    fn should_serde_proof_to_and_from_str() {
        let proof = get_sample_proof();
        let s = proof.to_string();
        let result = AlgorandTransactionProof::from_str(&s).unwrap();
        assert_eq!(result, proof);
    }

    #[test]
    fn should_verify_proof_1() {
        let proof = get_sample_proof();
        let tx_id = AlgorandHash::from_slice(
            &base32_decode("UFZTMQWJ3N6LWGMMSF7EJENOQKYYUDC7A2346TR3L7AYTBRCAPZQ").unwrap(),
        )
        .unwrap();
        // NOTE: curl -s "https://algoexplorerapi.io/v2/blocks/20261491" | jq .block.txn "YcZRhpAW/7OMq8q//Rm/fnuhZCBg5nhQwCTI0WNGbAI="
        let txn_root = AlgorandHash::from_slice(
            &base64_decode("YcZRhpAW/7OMq8q//Rm/fnuhZCBg5nhQwCTI0WNGbAI=").unwrap(),
        )
        .unwrap();
        let result = proof.is_valid(&tx_id, &txn_root).unwrap();
        assert!(result);
    }

    #[test]
    fn should_verify_proof_2() {
        // NOTE: Via: curl -s "https://testnet-api.algonode.cloud/v2/blocks/20827984/transactions/WFISCMEVNJQ44IGK5OSH767DGJW5E5JQK3HRJROIX3RVMXEDONOA/proof" | jq
        let proof = AlgorandTransactionProof::from_str(&json!(
            {
              "hashtype": "sha512_256",
              "idx": 2,
              "proof": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD7PrZCkE/aCgJQi2aM+FaMrBr75FnAMX8dI/6RLzFUwg==",
              "stibhash": "cYOXh02WbWew5J2x/vZApjQc2GaWP3Z/8/Lm3Rzzb2E=",
              "treedepth": 2
            }
        ).to_string()).unwrap();
        let tx_id = AlgorandHash::from_slice(
            &base32_decode("WFISCMEVNJQ44IGK5OSH767DGJW5E5JQK3HRJROIX3RVMXEDONOA").unwrap(),
        )
        .unwrap();
        // NOTE: curl -s "https://testnet-api.algonode.cloud/v2/blocks/20827984" | jq .block.txn
        let txn_root = AlgorandHash::from_slice(
            &base64_decode("qc2uzGvdxEV4ujtwkE4Jg5g+V3VMERmtKPBnxf91SNo=").unwrap(),
        )
        .unwrap();
        let result = proof.is_valid(&tx_id, &txn_root).unwrap();
        assert!(result);
    }

    #[test]
    fn should_verify_proof_3() {
        // NOTE: curl "https://testnet-api.algonode.cloud/v2/blocks/20827986/transactions/6JIBTA4NGUSGQONJRBJNU722S5PFZ3AGVU4VRY2ZIKR27AXBN6QQ/proof" | jq
        let proof = AlgorandTransactionProof::from_str(
            &json!({
              "hashtype": "sha512_256",
              "idx": 1,
              "proof": "GNGqyQvrIUZseT3msp90UY995OOnFkdHNuqmXeBUt9I=",
              "stibhash": "pb8iRzM057GCiY8Qp7MJh46mxHnMAO+oF1gEjPFAqRY=",
              "treedepth": 1
            })
            .to_string(),
        )
        .unwrap();
        let tx_id = AlgorandHash::from_slice(
            &base32_decode("6JIBTA4NGUSGQONJRBJNU722S5PFZ3AGVU4VRY2ZIKR27AXBN6QQ").unwrap(),
        )
        .unwrap();
        // NOTE: curl -s "https://testnet-api.algonode.cloud/v2/blocks/20827986" | jq .block.txn
        let txn_root = AlgorandHash::from_slice(
            &base64_decode("AZL7xI9Hp5DKWO59oHZPRmlE+wVoPOJQxIBuKJtWrbA=").unwrap(),
        )
        .unwrap();
        let result = proof.is_valid(&tx_id, &txn_root).unwrap();
        assert!(result);
    }

    #[test]
    fn should_verify_proof_4() {
        // NOTE curl "https://testnet-api.algonode.cloud/v2/blocks/20827988/transactions/S5UEAAO54HYPR3EPKWZRX3OE2GRKFOKCO2BUUICLQ2JEQBS4H5EQ/proof" | jq
        let proof = AlgorandTransactionProof::from_str(
            &json!({
              "hashtype": "sha512_256",
              "idx": 0,
              "proof": "",
              "stibhash": "8hi7qXsGUs5O80pOgGKXC5QYXso3sz8LLF1IoLeVvTE=",
              "treedepth": 0
            })
            .to_string(),
        )
        .unwrap();
        // NOTE: curl "https://testnet-api.algonode.cloud/v2/blocks/20827988" | jq .block.txn
        let txn_root = AlgorandHash::from_slice(
            &base64_decode("NNYUpJYQu30QVin14lFrri5tZjhDO+NLPWtXiWhgqqs=").unwrap(),
        )
        .unwrap();
        let tx_id = AlgorandHash::from_slice(
            &base32_decode("S5UEAAO54HYPR3EPKWZRX3OE2GRKFOKCO2BUUICLQ2JEQBS4H5EQ").unwrap(),
        )
        .unwrap();
        let result = proof.is_valid(&tx_id, &txn_root).unwrap();
        assert!(result);
    }

    #[test]
    fn should_verify_proof_5() {
        let proof = AlgorandTransactionProof::from_str(
            &json!({
              "hashtype": "sha512_256",
              "idx": 11,
              "treedepth": 5,
              "proof": "ab2/G0q7HayramFeLOdelgDgaVkLwY1XPZmilYNcTSsZJkbcCYgQSzBPF+sxCJYhsjXAORt0Cxx/+uYSO+Fo70kEcaNlD5kX4K18vOahWHKEg23bo0vPg4Hika8hUgldoIkff6mnXH9rbiDlBweVWY90VfPg7aq4ios5KR8TGgQhDMDraYncY0CL0gYA1gaTwp0J58Cdxz3GgJK+3ppGJg==",
              "stibhash": "jTXscHe2Wxyca8a2iwQZkxlDgCGC9ZRTJdGVTF1CLy4=",
            })
            .to_string(),
        )
        .unwrap();
        let txn_root = AlgorandHash::from_slice(
            &base64_decode("J/uRLt7jmzdA8o8Ju126ffKkhn5MFCQTbUsEQ3aZuSY=").unwrap(),
        )
        .unwrap();
        let tx_id = AlgorandHash::from_slice(
            &base32_decode("A3TATJKWNH4ZEYKDUZE4S5SO7TYIBQN5VSSWJB7HTFL4MJEAUOWQ").unwrap(),
        )
        .unwrap();
        let result = proof.is_valid(&tx_id, &txn_root).unwrap();
        assert!(result);
    }

    #[test]
    fn should_validate_proof_1() {}
}
