#![allow(clippy::too_many_arguments)]

use derive_more::Constructor;
use serde::Serialize;

mod asset_config_transaction;
mod asset_destroy_transaction;
mod asset_transfer_transaction;
mod pay_transaction;
mod transaction_test_utils;
mod transaction_type;

use crate::{
    algorand_address::AlgorandAddress,
    algorand_constants::ALGORAND_MAX_NUM_ROUNDS,
    algorand_hash::AlgorandHash,
    algorand_keys::AlgorandKeys,
    algorand_micro_algos::MICRO_ALGOS_MULTIPLIER,
    algorand_signature::AlgorandSignature,
    algorand_traits::ToMsgPackBytes,
    algorand_transaction::{
        asset_config_transaction::AssetParameters,
        transaction_type::AlgorandTransactionType,
    },
    algorand_types::{Byte, Bytes, Result},
    crypto_utils::{base32_encode_with_no_padding, sha512_256_hash_bytes},
};

impl ToMsgPackBytes for AlgorandTransaction {}
impl ToMsgPackBytes for AlgorandSignedTransaction {}

/// ## An Algorand Transaction
///
/// A struct holding the various fields required in an Algorand Transaction.
#[derive(Debug, Clone, Eq, PartialEq, Constructor, Serialize)]
pub struct AlgorandTransaction {
    /// ## Asset Amount
    ///
    /// The amount of an asset to transfer.
    #[serde(rename(serialize = "aamt"), skip_serializing_if = "Option::is_none")]
    asset_amount: Option<u64>,

    /// ## Amount
    ///
    /// The total amount to be sent in microAlgos.
    #[serde(rename(serialize = "amt"), skip_serializing_if = "Option::is_none")]
    amount: Option<u64>,

    /// ## Asset Parameters
    ///
    /// Asset paramets to include if the transaction is intended to create a new Algorand asset.
    #[serde(rename(serialize = "apar"), skip_serializing_if = "Option::is_none")]
    asset_parameters: Option<AssetParameters>,

    /// ## Asset Receiver
    ///
    /// The asset receiver in an asset transfer transaction.
    #[serde(rename(serialize = "arcv"), skip_serializing_if = "Option::is_none")]
    asset_receiver: Option<AlgorandAddress>,

    /// ## Asset ID
    ///
    /// An ID pointing to an asset on the Algorand blockchain.
    #[serde(rename(serialize = "caid"), skip_serializing_if = "Option::is_none")]
    asset_id: Option<u64>,

    /// ## Close Remainder To
    ///
    /// When set, it indicates that the tx is requesting that the sendng account should be closed.
    /// All remaining funds after the tx fee & amount are paid are be transferred to this address.
    #[serde(rename(serialize = "close"), skip_serializing_if = "Option::is_none")]
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
    #[serde(
        skip_serializing,
        rename(serialize = "gen"),
        skip_serializing_if = "Option::is_none"
    )]
    genesis_id: Option<String>,

    /// ## Genesis Hash
    ///
    /// The hash of the genesis block of the network on which the tx is valid.
    #[serde(rename(serialize = "gh"))]
    genesis_hash: AlgorandHash,

    /// ## Group
    ///
    /// The hash of the tx group this tx belongs to, if any.
    #[serde(rename(serialize = "group"), skip_serializing_if = "Option::is_none")]
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
    #[serde(rename(serialize = "rcv"), skip_serializing_if = "Option::is_none")]
    receiver: Option<AlgorandAddress>,

    /// ## RekeyTo
    ///
    /// Specifies the authorized address. This address will be used to authorize all future txs.
    #[serde(rename(serialize = "rekey"), skip_serializing_if = "Option::is_none")]
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
    txn_type: AlgorandTransactionType,

    /// ## Asset ID
    ///
    /// The unique ID of the asset to be transferred.
    #[serde(rename(serialize = "xaid"), skip_serializing_if = "Option::is_none")]
    transfer_asset_id: Option<u64>,
}

impl AlgorandTransaction {
    fn to_msg_pack_bytes(&self) -> Result<Bytes> {
        Ok(rmp_serde::to_vec_named(&self)?)
    }

    fn prefix_tx_byte(bytes: &[Byte]) -> Bytes {
        let suffix = bytes;
        let mut prefix = b"TX".to_vec();
        prefix.extend_from_slice(suffix);
        prefix
    }

    fn encode_for_signing(&self) -> Result<Bytes> {
        self.to_msg_pack_bytes()
            .map(|ref msg_pack_bytes| Self::prefix_tx_byte(msg_pack_bytes))
    }

    fn to_raw_tx_id(&self) -> Result<AlgorandHash> {
        AlgorandHash::from_slice(&sha512_256_hash_bytes(&self.encode_for_signing()?))
    }

    pub(crate) fn check_amount_is_above_minimum(amount: u64) -> Result<u64> {
        if amount >= MICRO_ALGOS_MULTIPLIER {
            Ok(amount)
        } else {
            Err(format!(
                "Amount is < minimum amount of {} algos!",
                MICRO_ALGOS_MULTIPLIER
            )
            .into())
        }
    }

    pub(crate) fn calculate_last_valid_round(
        first_valid_round: u64,
        last_valid_round: Option<u64>,
    ) -> Result<u64> {
        let last_round: u64 = match last_valid_round {
            None => first_valid_round + ALGORAND_MAX_NUM_ROUNDS,
            Some(last_valid_round_number) => {
                if last_valid_round_number <= first_valid_round {
                    return Err("Last valid round must be > than first valid round!".into());
                } else {
                    last_valid_round_number
                }
            },
        };
        if last_round > first_valid_round + ALGORAND_MAX_NUM_ROUNDS {
            return Err(format!(
                "Last valid round of {} is > {} away from first valid round of {}!",
                last_round, ALGORAND_MAX_NUM_ROUNDS, first_valid_round
            )
            .into());
        } else {
            Ok(last_round)
        }
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
    pub fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(self.to_msg_pack_bytes()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::AppError;

    #[test]
    fn amount_greater_than_minimum_should_pass_amount_check() {
        let amount = MICRO_ALGOS_MULTIPLIER + 1;
        let result = AlgorandTransaction::check_amount_is_above_minimum(amount);
        assert!(result.is_ok());
    }

    #[test]
    fn amount_less_than_than_minimum_should_fail_amount_check() {
        let amount = MICRO_ALGOS_MULTIPLIER - 1;
        let expected_error = format!(
            "Amount is < minimum amount of {} algos!",
            MICRO_ALGOS_MULTIPLIER
        );
        match AlgorandTransaction::check_amount_is_above_minimum(amount) {
            Ok(_) => panic!("Should not have succeeded!"),
            Err(AppError::Custom(error)) => assert_eq!(error, expected_error),
            Err(_) => panic!("Wrong error received!"),
        }
    }
}