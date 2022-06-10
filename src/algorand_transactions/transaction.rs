#![allow(clippy::too_many_arguments)]

use std::str::FromStr;

use base64::{decode as base64_decode, encode as base64_encode};
use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;

use crate::{
    algorand_address::AlgorandAddress,
    algorand_applications::algorand_application_args::AlgorandApplicationArg,
    algorand_constants::ALGORAND_MAX_NUM_ROUNDS,
    algorand_errors::AlgorandError,
    algorand_hash::AlgorandHash,
    algorand_keys::AlgorandKeys,
    algorand_micro_algos::MICRO_ALGOS_MULTIPLIER,
    algorand_signature::AlgorandSignature,
    algorand_traits::ToMsgPackBytes,
    algorand_transactions::{
        application_transaction::{ApplicationTransactionJson, OnCompletion},
        asset_config_transaction::AssetConfigTransactionJson,
        asset_freeze_transaction::AssetFreezeTransactionJson,
        asset_parameters::AssetParameters,
        asset_transfer_transaction::AssetTransferTransactionJson,
        key_reg_transaction::KeyRegTransactionJson,
        pay_transaction::PaymentTransactionJson,
        signature_json::AlgorandSignatureJson,
        transaction_json::AlgorandTransactionJson,
        transaction_type::AlgorandTransactionType,
    },
    algorand_types::{Byte, Bytes, Result},
    crypto_utils::{base32_encode_with_no_padding, sha512_256_hash_bytes},
};

impl ToMsgPackBytes for AlgorandTransaction {}
impl ToMsgPackBytes for AlgorandSignedTransaction {}

fn is_zero(num: &Option<u64>) -> bool {
    match num {
        Some(val) => val == &0,
        None => true,
    }
}

fn is_empty_vec<T>(vec: &Option<Vec<T>>) -> bool {
    match vec {
        Some(vec) => vec.is_empty(),
        None => true,
    }
}

/// ## An Algorand Transaction
///
/// A struct holding the various fields required in an Algorand Transaction.
#[skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, Default, Constructor, Serialize, Deserialize)]
pub struct AlgorandTransaction {
    /// ## Asset Amount
    ///
    /// The amount of an asset to transfer.
    #[serde(rename(serialize = "aamt"), skip_serializing_if = "is_zero")]
    pub asset_amount: Option<u64>,

    /// ## Asset Close To
    ///
    /// The address to send all remaining amount of asset to.
    #[serde(rename(serialize = "aclose"))]
    pub asset_close_to: Option<AlgorandAddress>,

    /// ## Asset Freeze Status
    ///
    /// The new freeze status of the asset.
    #[serde(rename(serialize = "afrz"))]
    pub asset_freeze_status: Option<bool>,

    /// ## Amount
    ///
    /// The total amount to be sent in microAlgos.
    #[serde(rename(serialize = "amt"), skip_serializing_if = "is_zero")]
    pub amount: Option<u64>,

    /// ## App Arguments
    ///
    /// Application arguments to be passed to the application being called
    #[serde(rename(serialize = "apaa"), skip_serializing_if = "is_empty_vec")]
    pub application_args: Option<Vec<AlgorandApplicationArg>>,

    /// ## On Completion
    ///
    /// OnCompletion specifies an optional side-effect that this transaction
    /// will have on the balance record of the sender or the application's
    /// creator. See the documentation for the OnCompletion type for more
    /// information on each possible value.
    #[serde(rename(serialize = "apan"), skip_serializing_if = "is_zero")]
    pub on_completion: Option<u64>,

    /// ## Asset Parameters
    ///
    /// Asset paramets to include if the transaction is intended to create a new Algorand asset.
    #[serde(rename(serialize = "apar"))]
    pub asset_parameters: Option<AssetParameters>,

    /// ## Foreign assets
    ///
    /// Asset IDs of assets that may be used by the application being called.
    #[serde(rename(serialize = "apas"), skip_serializing_if = "is_empty_vec")]
    pub foreign_assets: Option<Vec<u64>>,

    /// ## Accounts
    ///
    /// Account addresses of accounts that may be accessed by the application being called.
    #[serde(rename(serialize = "apat"), skip_serializing_if = "is_empty_vec")]
    pub accounts: Option<Vec<AlgorandAddress>>,

    /// ## Foreign applications
    ///
    /// Application IDs of applications that may be accessed by the application being called.
    #[serde(rename(serialize = "apfa"), skip_serializing_if = "is_empty_vec")]
    pub foreign_apps: Option<Vec<u64>>,

    /// ## Application ID
    ///
    /// The ID of an application to be called.
    #[serde(rename(serialize = "apid"))]
    pub application_id: Option<u64>,

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
    #[serde(rename(serialize = "caid"), skip_serializing_if = "is_zero")]
    // FIXME This is the config tx asset id! Add a prefix for clarity?
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
    #[serde(rename(serialize = "gen"))]
    pub genesis_id: Option<String>,

    /// ## Genesis Hash
    ///
    /// The hash of the genesis block of the network on which the tx is valid.
    #[serde(rename(serialize = "gh"))]
    pub genesis_hash: Option<AlgorandHash>,

    /// ## Group
    ///
    /// The hash of the tx group this tx belongs to, if any.
    #[serde(rename(serialize = "grp"))]
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

    // NOTE: These fields are retained when building tx from JSON
    #[serde(skip_serializing)]
    pub signature: Option<AlgorandSignature>,

    #[serde(skip_serializing)]
    pub asset_close_amount: Option<u64>,

    #[serde(skip_serializing)]
    pub close_amount: Option<u64>,

    #[serde(skip_serializing)]
    pub inner_txs: Option<Vec<AlgorandTransaction>>,

    #[serde(skip_serializing)]
    pub parent_tx_id: Option<AlgorandHash>,
}

impl FromStr for AlgorandTransaction {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_json(&serde_json::from_str(s)?)
    }
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

    pub fn assign_group_id(&self, group_id: AlgorandHash) -> Self {
        let mut mutable_self = self.clone();
        mutable_self.group = Some(group_id);
        mutable_self
    }

    pub fn assign_parent_id(&self, parent_id: AlgorandHash) -> Self {
        let mut mutable_self = self.clone();
        mutable_self.parent_tx_id = Some(parent_id);
        mutable_self
    }

    pub fn to_raw_tx_id(&self) -> Result<AlgorandHash> {
        AlgorandHash::from_slice(&sha512_256_hash_bytes(&self.encode_for_signing()?))
    }

    pub fn group(&self) -> Result<AlgorandHash> {
        match self.group {
            Some(hash) => Ok(hash),
            None => Err("No group ID set in transaction!".into()),
        }
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
            Err(format!(
                "Last valid round of {} is > {} away from first valid round of {}!",
                last_round, ALGORAND_MAX_NUM_ROUNDS, first_valid_round
            )
            .into())
        } else {
            Ok(last_round)
        }
    }

    /// ## To ID
    ///
    /// Calculate the transaction hash for this transaction.
    pub fn to_id(&self) -> Result<String> {
        match &self.parent_tx_id {
            Some(parent_tx_id) => Ok(base32_encode_with_no_padding(&parent_tx_id.to_bytes())),
            None => Ok(base32_encode_with_no_padding(
                &self.to_raw_tx_id()?.to_bytes(),
            )),
        }
    }

    /// ## Sign
    ///
    /// Sign the transaction with an AlgorandKey.
    pub fn sign(&self, keys: &AlgorandKeys) -> Result<AlgorandSignedTransaction> {
        let signer_address = keys.to_address()?;
        Ok(AlgorandSignedTransaction {
            transaction: self.clone(),
            transaction_id: Some(self.to_id()?),
            signature: keys.sign(&self.encode_for_signing()?),
            signer: match &self.sender {
                Some(sender) if signer_address != *sender => Some(signer_address),
                _ => None,
            },
        })
    }

    pub fn from_json(json: &AlgorandTransactionJson) -> Result<Self> {
        Ok(Self {
            fee: json.fee,
            amount: json.maybe_get_amount(),
            application_id: match &json.application_transaction {
                Some(app) => app.application_id,
                None => None,
            },
            on_completion: match &json.application_transaction {
                None => None,
                Some(app) => app.on_completion.as_ref().map(|thing| thing.to_u64()),
            },
            genesis_id: json.genesis_id.clone(),
            first_valid_round: json.first_valid,
            asset_id: json.maybe_get_config_asset_id(),
            asset_amount: json.maybe_get_asset_amount(),
            asset_close_to: match json.maybe_get_asset_close_to() {
                Some(address_str) => Some(AlgorandAddress::from_str(&address_str)?),
                None => None,
            },
            asset_receiver: match json.maybe_get_asset_receiver() {
                Some(address_str) => Some(AlgorandAddress::from_str(&address_str)?),
                None => None,
            },
            transfer_asset_id: json.maybe_get_transfer_asset_id(),
            asset_sender: match json.maybe_get_asset_sender() {
                Some(address_str) => Some(AlgorandAddress::from_str(&address_str)?),
                None => None,
            },
            txn_type: match &json.tx_type {
                Some(tx_type_str) => Some(AlgorandTransactionType::from_str(tx_type_str)?),
                None => None,
            },
            sender: match &json.sender {
                Some(address_str) => Some(AlgorandAddress::from_str(address_str)?),
                None => None,
            },
            genesis_hash: match &json.genesis_hash {
                Some(hash_str) => Some(AlgorandHash::from_str(hash_str)?),
                None => None,
            },
            group: match &json.group {
                Some(hash_str) => Some(AlgorandHash::from_str(hash_str)?),
                None => None,
            },
            last_valid_round: json.last_valid,
            lease: match &json.lease {
                Some(hash_str) => Some(AlgorandHash::from_str(hash_str)?),
                None => None,
            },
            note: match &json.note {
                Some(base64_str) => Some(base64_decode(&base64_str)?),
                None => None,
            },
            rekey_to: match &json.rekey_to {
                Some(address_str) => Some(AlgorandAddress::from_str(address_str)?),
                None => None,
            },
            asset_parameters: match &json.asset_config_transaction {
                Some(json) => Some(AssetParameters::from_json(&json.params)?),
                None => None,
            },
            receiver: match json.maybe_get_receiver() {
                Some(address_str) => Some(AlgorandAddress::from_str(&address_str)?),
                None => None,
            },
            close_remainder_to: match &json.close_remainder_to {
                Some(address_str) => Some(AlgorandAddress::from_str(address_str)?),
                None => None,
            },
            asset_freeze_id: match &json.asset_freeze_transaction {
                Some(freeze_tx) => freeze_tx.asset_id,
                None => None,
            },
            asset_freeze_address: match &json.asset_freeze_transaction {
                Some(freeze_tx) => match &freeze_tx.address {
                    Some(address_str) => Some(AlgorandAddress::from_str(address_str)?),
                    None => None,
                },
                None => None,
            },
            asset_freeze_status: match &json.asset_freeze_transaction {
                Some(freeze_tx) => freeze_tx.new_freeze_status,
                None => None,
            },
            asset_close_amount: match json.asset_transfer_transaction.as_ref() {
                Some(asset_transfer_json) => asset_transfer_json.close_amount,
                None => None,
            },
            signature: match json.signature.as_ref() {
                None => None,
                Some(sig_json) => match sig_json.sig.as_ref() {
                    Some(sig_str) => Some(AlgorandSignature::from_str(sig_str)?),
                    None => None,
                },
            },
            close_amount: match json.payment_transaction.as_ref() {
                Some(payment_json) => payment_json.close_amount,
                None => None,
            },
            application_args: match &json.application_transaction {
                Some(app) => Some(app.maybe_get_application_args()?),
                None => None,
            },
            foreign_apps: match &json.application_transaction {
                Some(app) => app.foreign_apps.clone(),
                None => None,
            },
            accounts: match &json.application_transaction {
                Some(app) => Some(app.maybe_get_accounts()?),
                None => None,
            },
            foreign_assets: match &json.application_transaction {
                Some(app) => app.foreign_assets.clone(),
                None => None,
            },
            inner_txs: match &json.id {
                Some(id) => match &json.inner_txs {
                    Some(inner_txs) => Some(
                        inner_txs
                            .iter()
                            .map(|tx| {
                                AlgorandTransaction::from_json(tx).and_then(|tx| {
                                    Ok(tx.assign_parent_id(AlgorandHash::from_base_32(id)?))
                                })
                            })
                            .collect::<Result<Vec<AlgorandTransaction>>>()?,
                    ),
                    None => None,
                },
                None => None,
            },
            parent_tx_id: match &json.parent_tx_id {
                Some(parent_tx_id) => Some(AlgorandHash::from_str(parent_tx_id)?),
                None => None,
            },
        })
    }

    pub fn to_json(&self) -> Result<AlgorandTransactionJson> {
        Ok(AlgorandTransactionJson {
            fee: self.fee,
            last_valid: self.last_valid_round,
            genesis_id: self.genesis_id.clone(),
            signature: self.to_signature_json(),
            first_valid: self.first_valid_round,
            id: Some(self.to_id()?),
            group: self.group.as_ref().map(|x| x.to_string()),
            lease: self.lease.as_ref().map(|x| x.to_string()),
            sender: self.sender.as_ref().map(|x| x.to_string()),
            tx_type: self.txn_type.as_ref().map(|x| x.to_string()),
            key_reg_transaction: self.to_key_ref_transaction_json(),
            rekey_to: self.rekey_to.as_ref().map(|x| x.to_string()),
            note: self.note.as_ref().map(|bytes| base64_encode(&bytes)),
            genesis_hash: self.genesis_hash.as_ref().map(|x| x.to_string()),
            asset_freeze_transaction: self.to_asset_freeze_transaction_json(),
            close_remainder_to: self.close_remainder_to.as_ref().map(|x| x.to_string()),
            asset_config_transaction: match &self.asset_parameters {
                None => None,
                Some(params) => Some(AssetConfigTransactionJson::new(
                    match &self.asset_id {
                        Some(id) => Result::Ok(*id),
                        None => Result::Ok(0),
                    }?,
                    params.to_json()?,
                )),
            },
            asset_transfer_transaction: self.to_asset_transfer_transaction_json(),
            payment_transaction: self.to_payment_transaction_json(),
            application_transaction: self.to_application_transaction_json()?,
            inner_txs: match &self.inner_txs {
                Some(inner_txs) => Some(
                    inner_txs
                        .iter()
                        .map(|tx| tx.to_json())
                        .collect::<Result<Vec<AlgorandTransactionJson>>>()?,
                ),
                _ => None,
            },
            parent_tx_id: self
                .parent_tx_id
                .as_ref()
                .map(|parent_tx_id| parent_tx_id.to_string()),
        })
    }

    fn to_signature_json(&self) -> Option<AlgorandSignatureJson> {
        let json = AlgorandSignatureJson {
            sig: self.signature.as_ref().map(|x| x.to_string()),
        };
        // FIXME: Do we need to check if empty?
        Some(json)
    }

    fn to_key_ref_transaction_json(&self) -> Option<KeyRegTransactionJson> {
        // FIXME Impl this! Check if empty too!
        None
    }

    fn to_application_transaction_json(&self) -> Result<Option<ApplicationTransactionJson>> {
        let json = ApplicationTransactionJson {
            accounts: match &self.accounts {
                Some(accounts) if !accounts.is_empty() => Some(
                    accounts
                        .iter()
                        .map(|account| account.to_base32())
                        .collect::<Result<Vec<String>>>()?,
                ),
                _ => None,
            },
            foreign_apps: self.foreign_apps.clone(),
            on_completion: match self.on_completion {
                Some(val) => OnCompletion::from_u64(val).ok(),
                None => None,
            },
            foreign_assets: self.foreign_assets.clone(),
            application_id: self.application_id,
            application_args: match &self.application_args {
                Some(args) if !args.is_empty() => Some(args.iter().map(base64_encode).collect()),
                _ => None,
            },
            approval_program: None,
            clear_state_program: None,
            global_state_schema: None,
            local_state_schema: None,
        };
        if json.is_empty() {
            Ok(None)
        } else {
            Ok(Some(json))
        }
    }

    fn to_asset_transfer_transaction_json(&self) -> Option<AssetTransferTransactionJson> {
        let json = AssetTransferTransactionJson {
            asset_id: self.transfer_asset_id,
            close_amount: self.asset_close_amount,
            sender: self.asset_sender.as_ref().map(|x| x.to_string()),
            amount: self.asset_amount.as_ref().map(|u_64| json!(u_64)),
            receiver: self.asset_receiver.as_ref().map(|x| x.to_string()),
            close_to: self.asset_close_to.as_ref().map(|x| x.to_string()),
        };
        if json.is_empty() {
            None
        } else {
            Some(json)
        }
    }

    fn to_payment_transaction_json(&self) -> Option<PaymentTransactionJson> {
        let json = PaymentTransactionJson {
            close_amount: self.close_amount,
            amount: self.amount.as_ref().map(|u_64| json!(u_64)),
            receiver: self.receiver.as_ref().map(|x| x.to_string()),
        };
        if json.is_empty() {
            None
        } else {
            Some(json)
        }
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
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AlgorandSignedTransaction {
    #[serde(rename(serialize = "sgnr"))]
    pub signer: Option<AlgorandAddress>,

    #[serde(rename(serialize = "sig"))]
    pub signature: AlgorandSignature,

    #[serde(rename(serialize = "txn"))]
    pub transaction: AlgorandTransaction,

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
impl AlgorandTransaction {
    pub fn find_difference(&self, other: &AlgorandTransaction) {
        use paste::paste;
        let mut err: String;
        macro_rules! assert_equality {
            ($($field:expr),*) => {
                paste! {
                    $(
                        err = format!("'{}' field  does not match!", $field);
                        assert_eq!(self.[< $field >], other.[< $field >], "{}", err);
                    )*
                }
            }
        }
        assert_equality!(
            "asset_amount",
            "asset_close_to",
            "asset_freeze_status",
            "amount",
            "asset_parameters",
            "asset_receiver",
            "asset_sender",
            "asset_id",
            "close_remainder_to",
            "asset_freeze_address",
            "asset_freeze_id",
            "fee",
            "first_valid_round",
            "genesis_id",
            "genesis_hash",
            "group",
            "last_valid_round",
            "lease",
            "note",
            "receiver",
            "rekey_to",
            "sender",
            "txn_type",
            "transfer_asset_id"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        algorand_errors::AlgorandError,
        algorand_transactions::test_utils::{get_sample_txs_jsons, get_sample_txs_n},
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
            let tx = AlgorandTransaction::from_json(json).unwrap();
            assert_eq!(json.id.as_ref().unwrap(), &tx.to_id().unwrap())
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
        jsons
            .iter()
            .zip(results.iter())
            .for_each(|(json_before, json_after)| json_before.assert_equality(json_after))
    }

    #[test]
    fn should_calculate_inner_tx_id_correctly() {
        let txs = get_sample_txs_n(2);
        let tx = txs
            .iter()
            .filter(|tx| {
                tx.txn_type == Some(AlgorandTransactionType::ApplicationCall)
                    && tx.inner_txs.is_some()
            })
            .cloned()
            .collect::<Vec<AlgorandTransaction>>()[0]
            .clone()
            .inner_txs
            .unwrap()[0]
            .clone();
        let result = tx.to_id().unwrap();
        let expected_result = "5IBWPOEE3ZB7UBO3G42T34YVPVK6CT32T46HHECESD5BJC5SVK3A";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_calculate_inner_tx_id_correctly_after_json_serialization() {
        let txs = get_sample_txs_n(2);
        let tx = txs
            .iter()
            .filter(|tx| {
                tx.txn_type == Some(AlgorandTransactionType::ApplicationCall)
                    && tx.inner_txs.is_some()
            })
            .cloned()
            .collect::<Vec<AlgorandTransaction>>()[0]
            .clone()
            .inner_txs
            .unwrap()[0]
            .clone();
        let json = tx.to_json().unwrap();
        let tx_from_bytes = AlgorandTransaction::from_json(&json).unwrap();
        let result = tx_from_bytes.to_id().unwrap();
        let expected_result = "5IBWPOEE3ZB7UBO3G42T34YVPVK6CT32T46HHECESD5BJC5SVK3A";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_calculate_inner_tx_id_correctly_after_json_bytes_serialization() {
        let txs = get_sample_txs_n(2);
        let tx = txs
            .iter()
            .filter(|tx| {
                tx.txn_type == Some(AlgorandTransactionType::ApplicationCall)
                    && tx.inner_txs.is_some()
            })
            .cloned()
            .collect::<Vec<AlgorandTransaction>>()[0]
            .clone()
            .inner_txs
            .unwrap()[0]
            .clone();
        let bytes = tx.to_json().unwrap().to_bytes().unwrap();
        let tx_from_bytes =
            AlgorandTransaction::from_json(&AlgorandTransactionJson::from_bytes(&bytes).unwrap())
                .unwrap();
        let result = tx_from_bytes.to_id().unwrap();
        let expected_result = "5IBWPOEE3ZB7UBO3G42T34YVPVK6CT32T46HHECESD5BJC5SVK3A";
        assert_eq!(result, expected_result);
    }
}
