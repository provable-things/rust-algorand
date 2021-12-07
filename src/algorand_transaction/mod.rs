#![allow(clippy::too_many_arguments)]

use derive_more::Constructor;
use ed25519_dalek::Signature;
use serde::Serialize;

use crate::{
    algorand_address::AlgorandAddress,
    algorand_hash::AlgorandHash,
    algorand_keys::AlgorandKeys,
    algorand_micro_algos::MicroAlgos,
    algorand_signature::AlgorandSignature,
    algorand_traits::ToMsgPackBytes,
    algorand_types::{Byte, Bytes, Result},
    constants::ALGORAND_MAX_NUM_ROUNDS,
    crypto_utils::{base32_encode_with_no_padding, sha512_256_hash_bytes},
};

impl ToMsgPackBytes for AlgorandTransaction {}
impl ToMsgPackBytes for AlgorandSignedTransaction {}

/// ## An Algorand Tx
///
/// A struct holding the various fields required in an Algorand Transaction.
#[derive(Debug, Clone, Eq, PartialEq, Constructor, Serialize)]
pub struct AlgorandTransaction {
    /// ## Amount
    ///
    /// The total amount to be sent in microAlgos.
    #[serde(rename(serialize = "amt"))]
    amount: u64,

    /// ## Close Remainder To
    ///
    /// When set, it indicates that the tx is requesting that the sendng account should be closed.
    /// All remaining funds after the tx fee & amount are paid are be transferred to this address.
    #[serde(rename(serialize = "close"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    close_remainder_to: Option<AlgorandAddress>,

    /// ## Fee
    ///
    /// Paid by the sender to the `FeeSink` account to prevent denial-of-service attacks.
    /// The minimum at time of writing is 1000 MicroAlgos.
    fee: u64,

    /// ## First Valid Round
    ///
    /// The first round after which the tx is valid.
    #[serde(rename(serialize = "fv"))]
    first_valid_round: u64,

    /// ## Genesis ID
    ///
    /// The human-readable form of the genesis hash.
    #[serde(rename(serialize = "gen"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_serializing)]
    genesis_id: Option<String>,

    /// ## Genesis Hash
    ///
    /// The hash of the genesis block of the network on which the tx is valid.
    #[serde(rename(serialize = "gh"))]
    genesis_hash: AlgorandHash,

    /// ## Group
    ///
    /// The hash of the tx group this tx belongs to, if any.
    #[serde(rename(serialize = "group"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<AlgorandHash>,

    /// ## Last Valid Round
    ///
    /// The last round after which the tx is no longer valid.
    #[serde(rename(serialize = "lv"))]
    last_valid_round: u64,

    /// ## Lease
    ///
    /// A lease enforces mutual exclusion of txs. If this field is nonzero, then once the
    /// tx is confirmed, it acquires the lease identified by the (Sender, Lease) pair of
    /// the tx until the LastValid round passes. While this tx possesses the
    /// lease, no other tx specifying this lease can be confirmed. A lease is often used
    /// in the context of Algorand Smart Contracts to prevent replay attacks. Read more about
    /// Algorand Smart Contracts and see the Delegate Key Registration TEAL template for an example
    /// implementation of leases. Leases can also be used to safeguard against unintended duplicate
    /// spends. For example, if I send a tx to the network and later realize my fee was too
    /// low, I could send another tx with a higher fee, but the same lease value. This would
    /// ensure that only one of those txs ends up getting confirmed during the validity period.
    #[serde(skip_serializing_if = "Option::is_none")]
    lease: Option<AlgorandHash>,

    /// # Note
    /// #
    /// Any data up to 1000 bytes.
    #[serde(
        rename = "note",
        with = "serde_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    note: Option<Bytes>,

    /// ## Receiver
    ///
    /// The address of the account whom receives the amount.
    #[serde(rename(serialize = "rcv"))]
    receiver: AlgorandAddress,

    /// ## RekeyTo
    ///
    /// Specifies the authorized address. This address will be used to authorize all future txs.
    #[serde(rename(serialize = "rekey"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    rekey_to: Option<AlgorandAddress>,

    /// ## Sender
    ///
    /// The address of the account which signs the tx and pays the fee & amount.
    #[serde(rename(serialize = "snd"))]
    sender: AlgorandAddress,

    /// ## Txn Type
    ///
    /// Specifies the type of tx.
    #[serde(rename(serialize = "type"))]
    txn_type: String,
}

impl AlgorandTransaction {
    fn calculate_last_valid_round(
        first_valid_round: u64,
        last_valid_round: Option<u64>,
    ) -> Result<u64> {
        match last_valid_round {
            None => Ok(first_valid_round + ALGORAND_MAX_NUM_ROUNDS),
            Some(last_valid_round_number) => {
                if last_valid_round_number <= first_valid_round {
                    Err("Last valid round must be > than first valid round!".into())
                } else {
                    Ok(last_valid_round_number)
                }
            },
        }
    }

    /// ## New Payment Transaction
    ///
    /// Create a new, simple payment transaction with an optional note.
    pub fn new_payment_tx(
        amount: u64, // FIXME check this is above 100,000 minimum!
        fee: MicroAlgos,
        note: Option<Bytes>,
        first_valid_round: u64,
        sender: AlgorandAddress,
        receiver: AlgorandAddress,
        genesis_hash: AlgorandHash,
        last_valid_round: Option<u64>,
    ) -> Result<Self> {
        let calculated_last_valid_round =
            Self::calculate_last_valid_round(first_valid_round, last_valid_round)?;
        if calculated_last_valid_round > first_valid_round + ALGORAND_MAX_NUM_ROUNDS {
            return Err(format!(
                "Last valid round of {} is > {} away from first valid round of {}!",
                calculated_last_valid_round, ALGORAND_MAX_NUM_ROUNDS, first_valid_round
            )
            .into());
        };
        Ok(Self {
            note,
            sender,
            amount,
            receiver,
            genesis_hash,
            first_valid_round,
            txn_type: "pay".to_string(),
            fee: fee.check_if_satisfies_minimum_fee()?.0,
            last_valid_round: calculated_last_valid_round,
            group: None,
            lease: None,
            rekey_to: None,
            genesis_id: None,
            close_remainder_to: None,
        })
    }

    fn to_msg_pack_bytes(&self) -> Result<Bytes> {
        Ok(rmp_serde::to_vec_named(&self)?)
    }

    fn prefix_tx_byte(bytes: &[Byte]) -> Bytes {
        let mut suffix = bytes.clone();
        let mut prefix = b"TX".to_vec();
        prefix.extend_from_slice(&suffix);
        prefix
    }

    fn encode_for_signing(&self) -> Result<Bytes> {
        self.to_msg_pack_bytes()
            .map(|ref msg_pack_bytes| Self::prefix_tx_byte(msg_pack_bytes))
    }

    fn to_raw_tx_id(&self) -> Result<AlgorandHash> {
        AlgorandHash::from_slice(&sha512_256_hash_bytes(&self.encode_for_signing()?))
    }

    /// ## To ID
    ///
    /// Calculate the transaction hash for this transaction.
    fn to_id(&self) -> Result<String> {
        Ok(base32_encode_with_no_padding(
            &self.to_raw_tx_id()?.to_bytes(),
        ))
    }

    /// ## Sign
    ///
    /// Sign the transaction with an AlgorandKey.
    pub fn sign(&self, keys: &AlgorandKeys) -> Result<AlgorandSignedTransaction> {
        Ok(AlgorandSignedTransaction {
            transaction: self.clone(),
            transaction_id: Some(self.to_id()?),
            signature: keys.sign(&self.encode_for_signing()?),
        })
    }
}

/// ## Algorand Signed Transaction
///
/// A struct to hold a signed algorand transaction, in a format which when serialized is able to be
/// broadcast to the algorand network.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AlgorandSignedTransaction {
    #[serde(rename(serialize = "sig"))]
    signature: AlgorandSignature,

    #[serde(rename(serialize = "txn"))]
    transaction: AlgorandTransaction,

    #[serde(skip_serializing)]
    #[serde(rename(serialize = "txid"))]
    transaction_id: Option<String>,
}

impl AlgorandSignedTransaction {
    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(self.to_msg_pack_bytes()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        constants::ALGORAND_MAINNET_GENESIS_ID,
        errors::AppError,
        test_utils::get_sample_algorand_keys,
    };

    fn get_sample_tx() -> AlgorandTransaction {
        let first_valid_round = 1000;
        let note = None;
        let last_valid_round = None;
        AlgorandTransaction::new_payment_tx(
            1337,
            MicroAlgos::minimum_fee(),
            note,
            first_valid_round,
            AlgorandAddress::from_str("4IZRTUO72JY5WH4HKLVDQSKIVF2VSRQX7IFVI3KEOQHHNCQUXCMYPZH7J4")
                .unwrap(),
            AlgorandAddress::from_str("GULDQIEZ2CUPBSHKXRWUW7X3LCYL44AI5GGSHHOQDGKJAZ2OANZJ43S72U")
                .unwrap(),
            AlgorandHash::mainnet_genesis_hash().unwrap(),
            last_valid_round,
        )
        .unwrap()
    }

    #[test]
    fn should_encode_tx_to_msg_pack_bytes() {
        let tx = get_sample_tx();
        let result = hex::encode(tx.to_msg_pack_bytes().unwrap());
        let expected_result = "88a3616d74cd0539a3666565cd03e8a26676cd03e8a26768c420c061c4d8fc1dbdded2d7604be4568e3f6d041987ac37bde4b620b5ab39248adfa26c76cd07d0a3726376c4203516382099d0a8f0c8eabc6d4b7efb58b0be7008e98d239dd0199490674e0372a3736e64c420e23319d1dfd271db1f8752ea384948a975594617fa0b546d44740e768a14b899a474797065a3706179";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_encode_tx_for_signing() {
        let tx = get_sample_tx();
        let result = hex::encode(tx.encode_for_signing().unwrap());
        let expected_result = "545888a3616d74cd0539a3666565cd03e8a26676cd03e8a26768c420c061c4d8fc1dbdded2d7604be4568e3f6d041987ac37bde4b620b5ab39248adfa26c76cd07d0a3726376c4203516382099d0a8f0c8eabc6d4b7efb58b0be7008e98d239dd0199490674e0372a3736e64c420e23319d1dfd271db1f8752ea384948a975594617fa0b546d44740e768a14b899a474797065a3706179";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_get_transaction_hash() {
        let tx = get_sample_tx();
        let result = tx.to_id().unwrap();
        let expected_result = "CT3DHZ5ZK6VZWXIMDZXAVIVH6DA4V26HWBPCHIZGHLVWFEOC7P6A";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_sign_tx() {
        let tx = get_sample_tx();
        let keys = get_sample_algorand_keys();
        let signed_tx = tx.sign(&keys).unwrap();
        let result = hex::encode(signed_tx.to_msg_pack_bytes().unwrap());
        let expected_result = "82a3736967c440a6e1b839f2f0109afa914694264e1fcee2ce3b5858ff46c436a69ece9f9630d9d182b1307f4bb8a1807e785326f91beded1a6d1a2368c1e0644b58bf6bc60f06a374786e88a3616d74cd0539a3666565cd03e8a26676cd03e8a26768c420c061c4d8fc1dbdded2d7604be4568e3f6d041987ac37bde4b620b5ab39248adfa26c76cd07d0a3726376c4203516382099d0a8f0c8eabc6d4b7efb58b0be7008e98d239dd0199490674e0372a3736e64c420e23319d1dfd271db1f8752ea384948a975594617fa0b546d44740e768a14b899a474797065a3706179";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_calculate_last_valid_round_if_none_given() {
        let first_valid_round = 1000;
        let result =
            AlgorandTransaction::calculate_last_valid_round(first_valid_round, None).unwrap();
        let expected_result = 2000;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_error_if_last_valid_round_lte_first_valid_round() {
        let first_valid_round = 1000;
        let last_valid_round = first_valid_round - 1;
        let expected_error = "Last valid round must be > than first valid round!";
        match AlgorandTransaction::calculate_last_valid_round(
            first_valid_round,
            Some(last_valid_round),
        ) {
            Ok(_) => panic!("Should not have succeeded!"),
            Err(AppError::Custom(error)) => assert_eq!(error, expected_error),
            Err(_) => panic!("Wrong error received"),
        }
    }

    #[test]
    fn should_use_last_valid_round_if_valid() {
        let first_valid_round = 1000;
        let last_valid_round = 1001;
        let result = AlgorandTransaction::calculate_last_valid_round(
            first_valid_round,
            Some(last_valid_round),
        )
        .unwrap();
        assert_eq!(result, last_valid_round);
    }
}
