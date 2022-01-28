#![allow(clippy::too_many_arguments)]

use std::str::FromStr;

use base64::{decode as base64_decode, encode as base64_encode};
use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    algorand_address::AlgorandAddress,
    algorand_constants::ALGORAND_MAX_NUM_ROUNDS,
    algorand_errors::AlgorandError,
    algorand_hash::AlgorandHash,
    algorand_keys::AlgorandKeys,
    algorand_micro_algos::MICRO_ALGOS_MULTIPLIER,
    algorand_signature::AlgorandSignature,
    algorand_traits::ToMsgPackBytes,
    algorand_transactions::{
        asset_config_transaction::AssetConfigTransactionJson,
        asset_freeze_transaction::AssetFreezeTransactionJson,
        asset_parameters::AssetParameters,
        transaction_json::AlgorandTransactionJson,
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
#[skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, Default, Constructor, Serialize, Deserialize)]
pub struct AlgorandTransaction {
    /// ## Asset Amount
    ///
    /// The amount of an asset to transfer.
    #[serde(rename(serialize = "aamt"))]
    pub asset_amount: Option<u64>,

    /// ## Asset Freeze Status
    ///
    /// The new freeze status of the asset.
    #[serde(rename(serialize = "afrz"))]
    pub asset_freeze_status: Option<bool>,

    /// ## Amount
    ///
    /// The total amount to be sent in microAlgos.
    #[serde(rename(serialize = "amt"))]
    pub amount: Option<u64>,

    /// ## Asset Parameters
    ///
    /// Asset paramets to include if the transaction is intended to create a new Algorand asset.
    #[serde(rename(serialize = "apar"))]
    pub asset_parameters: Option<AssetParameters>,

    /// ## Asset Receiver
    ///
    /// The asset receiver in an asset transfer transaction.
    #[serde(rename(serialize = "arcv"))]
    pub asset_receiver: Option<AlgorandAddress>,

    /// ## Asset Sender
    ///
    /// The address from which the funds will be clawed back from.
    #[serde(rename(serialize = "asnd"))]
    pub asset_sender: Option<AlgorandAddress>,

    /// ## Asset ID
    ///
    /// An ID pointing to an asset on the Algorand blockchain.
    #[serde(rename(serialize = "caid"))]
    pub asset_id: Option<u64>,

    /// ## Close Remainder To
    ///
    /// When set, it indicates that the tx is requesting that the sendng account should be closed.
    /// All remaining funds after the tx fee & amount are paid are be transferred to this address.
    #[serde(rename(serialize = "close"))]
    pub close_remainder_to: Option<AlgorandAddress>,

    /// ## Asset Freeze Address
    ///
    /// Address of the account whose asset is being frozen or thawed.
    #[serde(rename(serialize = "fadd"))]
    pub asset_freeze_address: Option<AlgorandAddress>,

    /// ## Asset Freeze ID
    ///
    /// ID of the asset being frozen or thawed.
    #[serde(rename(serialize = "faid"))]
    pub asset_freeze_id: Option<u64>,

    /// ## Fee
    ///
    /// Paid by the sender to the `FeeSink` account to prevent denial-of-service attacks.
    /// The minimum at time of writing is 1000 MicroAlgos.
    pub fee: Option<u64>,

    /// ## First Valid Round
    ///
    /// The first round after which the tx is valid.
    #[serde(rename(serialize = "fv"))]
    pub first_valid_round: Option<u64>,

    /// ## Genesis ID
    ///
    /// The human-readable form of the genesis hash.
    #[serde(skip_serializing, rename(serialize = "gen"))]
    pub genesis_id: Option<String>,

    /// ## Genesis Hash
    ///
    /// The hash of the genesis block of the network on which the tx is valid.
    #[serde(rename(serialize = "gh"))]
    pub genesis_hash: Option<AlgorandHash>,

    /// ## Group
    ///
    /// The hash of the tx group this tx belongs to, if any.
    #[serde(rename(serialize = "group"))]
    pub group: Option<AlgorandHash>,

    /// ## Last Valid Round
    ///
    /// The last round after which the tx is no longer valid.
    #[serde(rename(serialize = "lv"))]
    pub last_valid_round: Option<u64>,

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
    pub lease: Option<AlgorandHash>,

    /// # Note
    /// #
    /// Any data up to 1000 bytes.
    #[serde(with = "serde_bytes")]
    pub note: Option<Bytes>,

    /// ## Receiver
    ///
    /// The address of the account whom receives the amount.
    #[serde(rename(serialize = "rcv"))]
    pub receiver: Option<AlgorandAddress>,

    /// ## RekeyTo
    ///
    /// Specifies the authorized address. This address will be used to authorize all future txs.
    #[serde(rename(serialize = "rekey"))]
    pub rekey_to: Option<AlgorandAddress>,

    /// ## Sender
    ///
    /// The address of the account which signs the tx and pays the fee & amount.
    #[serde(rename(serialize = "snd"))]
    pub sender: Option<AlgorandAddress>,

    /// ## Txn Type
    ///
    /// Specifies the type of tx.
    #[serde(rename(serialize = "type"))]
    pub txn_type: Option<AlgorandTransactionType>,

    /// ## Asset ID
    ///
    /// The unique ID of the asset to be transferred.
    #[serde(rename(serialize = "xaid"))]
    pub transfer_asset_id: Option<u64>,
}

impl AlgorandTransaction {
    /// ## To Bytes
    ///
    /// Convert the transaction to its msgpack-ed bytes.
    fn to_bytes(&self) -> Result<Bytes> {
        self.to_msg_pack_bytes()
    }

    fn to_msg_pack_bytes(&self) -> Result<Bytes> {
        Ok(rmp_serde::to_vec_named(&self)?)
    }

    fn prefix_tx_byte(bytes: &[Byte]) -> Bytes {
        let suffix = bytes;
        let mut prefix = b"TX".to_vec();
        prefix.extend_from_slice(suffix);
        prefix
    }

    pub fn encode_for_signing(&self) -> Result<Bytes> {
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
    pub fn to_id(&self) -> Result<String> {
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

    pub fn from_json(json: &AlgorandTransactionJson) -> Result<Self> {
        Ok(Self {
            fee: json.fee,
            amount: json.amount.clone(),
            asset_id: json.asset_id.clone(),
            genesis_id: json.genesis_id.clone(),
            first_valid_round: json.first_valid,
            asset_amount: json.asset_amount.clone(),
            transfer_asset_id: json.transfer_asset_id.clone(),
            txn_type: match &json.tx_type {
                Some(tx_type_str) => Some(AlgorandTransactionType::from_str(&tx_type_str)?),
                None => None,
            },
            sender: match &json.sender {
                Some(address_str) => Some(AlgorandAddress::from_str(&address_str)?),
                None => None,
            },
            asset_sender: match &json.asset_sender {
                Some(address_str) => Some(AlgorandAddress::from_str(&address_str)?),
                None => None,
            },
            genesis_hash: match &json.genesis_hash {
                Some(hash_str) => Some(AlgorandHash::from_str(&hash_str)?),
                None => None,
            },
            group: match &json.group {
                Some(hash_str) => Some(AlgorandHash::from_str(&hash_str)?),
                None => None,
            },
            last_valid_round: json.last_valid,
            lease: match &json.lease {
                Some(hash_str) => Some(AlgorandHash::from_str(&hash_str)?),
                None => None,
            },
            note: match &json.note {
                Some(base64_str) => Some(base64_decode(&base64_str)?),
                None => None,
            },
            rekey_to: match &json.rekey_to {
                Some(address_str) => Some(AlgorandAddress::from_str(&address_str)?),
                None => None,
            },
            asset_parameters: match &json.asset_config_transaction {
                Some(json) => Some(AssetParameters::from_json(&json.params)?),
                None => None,
            },
            asset_receiver: match &json.asset_receiver {
                Some(address_str) => Some(AlgorandAddress::from_str(&address_str)?),
                None => None,
            },
            receiver: match &json.receiver {
                Some(address_str) => Some(AlgorandAddress::from_str(&address_str)?),
                None => None,
            },
            close_remainder_to: match &json.close_remainder_to {
                Some(address_str) => Some(AlgorandAddress::from_str(&address_str)?),
                None => None,
            },
            asset_freeze_id: match &json.asset_freeze_transaction {
                Some(freeze_tx) => freeze_tx.asset_id,
                None => None,
            },
            asset_freeze_address: match &json.asset_freeze_transaction {
                Some(freeze_tx) => match &freeze_tx.address {
                    Some(address_str) => Some(AlgorandAddress::from_str(&address_str)?),
                    None => None,
                },
                None => None,
            },
            asset_freeze_status: match &json.asset_freeze_transaction {
                Some(freeze_tx) => freeze_tx.new_freeze_status,
                None => None,
            },
        })
    }

    pub fn to_json(&self) -> Result<AlgorandTransactionJson> {
        Ok(AlgorandTransactionJson {
            fee: self.fee.clone(),
            amount: self.amount.clone(),
            asset_id: self.asset_id.clone(),
            genesis_id: self.genesis_id.clone(),
            asset_amount: self.asset_amount.clone(),
            last_valid: self.last_valid_round.clone(),
            first_valid: self.first_valid_round.clone(),
            transfer_asset_id: self.transfer_asset_id.clone(),
            group: self.group.as_ref().map(|x| x.to_string()),
            lease: self.lease.as_ref().map(|x| x.to_string()),
            sender: self.sender.as_ref().map(|x| x.to_string()),
            tx_type: self.txn_type.as_ref().map(|x| x.to_string()),
            receiver: self.receiver.as_ref().map(|x| x.to_string()),
            rekey_to: self.rekey_to.as_ref().map(|x| x.to_string()),
            note: self.note.as_ref().map(|bytes| base64_encode(&bytes)),
            asset_sender: self.asset_sender.as_ref().map(|x| x.to_string()),
            genesis_hash: self.genesis_hash.as_ref().map(|x| x.to_string()),
            asset_freeze_transaction: self.to_asset_freeze_transaction_json(),
            asset_receiver: self.asset_receiver.as_ref().map(|x| x.to_string()),
            close_remainder_to: self.close_remainder_to.as_ref().map(|x| x.to_string()),
            asset_config_transaction: match &self.asset_parameters {
                None => None,
                Some(params) => Some(AssetConfigTransactionJson::new(
                    match &self.asset_id {
                        Some(id) => Result::Ok(*id),
                        // FIXME
                        //None => Result::Err("Tx with asset config params but no asset
                        // ID!".into())
                        None => Result::Ok(0),
                    }?,
                    params.to_json()?,
                )),
            },
        })
    }

    fn to_asset_freeze_transaction_json(&self) -> Option<AssetFreezeTransactionJson> {
        let json = AssetFreezeTransactionJson {
            asset_id: self.asset_freeze_id,
            new_freeze_status: self.asset_freeze_status,
            address: self.asset_freeze_address.as_ref().map(|x| x.to_string()),
        };
        if json.is_empty() {
            None
        } else {
            Some(json)
        }
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
    use crate::{
        algorand_errors::AlgorandError,
        algorand_transactions::test_utils::get_sample_txs_jsons,
    };

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
            Err(AlgorandError::Custom(error)) => assert_eq!(error, expected_error),
            Err(_) => panic!("Wrong error received!"),
        }
    }

    #[test]
    fn should_get_algorand_transactions_from_jsons() {
        let jsons = get_sample_txs_jsons(0);
        jsons.iter().for_each(|json| {
            if AlgorandTransaction::from_json(json).is_err() {
                println!("JSON which failed to parse: {:?}", json);
            }
            AlgorandTransaction::from_json(json).unwrap();
        });
    }

    #[test]
    fn should_serde_algorand_transactions_to_and_from_json() {
        let jsons = get_sample_txs_jsons(0);
        let txs = jsons
            .iter()
            .map(|json| AlgorandTransaction::from_json(&json))
            .collect::<Result<Vec<AlgorandTransaction>>>()
            .unwrap();
        let results = txs
            .iter()
            .map(|tx| tx.to_json())
            .collect::<Result<Vec<AlgorandTransactionJson>>>()
            .unwrap();
        results.iter().enumerate().for_each(|(i, json)| {
            assert_eq!(json.fee, jsons[i].fee);
            assert_eq!(json.note, jsons[i].note);
            assert_eq!(json.group, jsons[i].group);
            assert_eq!(json.lease, jsons[i].lease);
            assert_eq!(json.amount, jsons[i].amount);
            assert_eq!(json.sender, jsons[i].sender);
            assert_eq!(json.tx_type, jsons[i].tx_type);
            assert_eq!(json.receiver, jsons[i].receiver);
            assert_eq!(json.rekey_to, jsons[i].rekey_to);
            assert_eq!(json.asset_id, jsons[i].asset_id);
            assert_eq!(json.genesis_id, jsons[i].genesis_id);
            assert_eq!(json.last_valid, jsons[i].last_valid);
            assert_eq!(json.first_valid, jsons[i].first_valid);
            assert_eq!(json.asset_amount, jsons[i].asset_amount);
            assert_eq!(json.asset_sender, jsons[i].asset_sender);
            assert_eq!(json.genesis_hash, jsons[i].genesis_hash);
            assert_eq!(json.asset_receiver, jsons[i].asset_receiver);
            assert_eq!(json.transfer_asset_id, jsons[i].transfer_asset_id);
            assert_eq!(json.close_remainder_to, jsons[i].close_remainder_to);
            assert_eq!(
                json.asset_config_transaction,
                jsons[i].asset_config_transaction
            );
            assert_eq!(
                json.asset_freeze_transaction,
                jsons[i].asset_freeze_transaction,
            );
        })
    }
}
