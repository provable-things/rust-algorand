use std::{fmt::Display, str::FromStr};

use base64::decode as base64_decode;
use serde::{Deserialize, Serialize};

use crate::{
    algorand_address::AlgorandAddress,
    algorand_applications::algorand_application_args::AlgorandApplicationArg,
    algorand_errors::AlgorandError,
    algorand_hash::AlgorandHash,
    algorand_micro_algos::MicroAlgos,
    algorand_transactions::{
        transaction::AlgorandTransaction,
        transaction_type::AlgorandTransactionType,
    },
    algorand_types::Result,
};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum OnCompletion {
    #[serde(rename = "noop")]
    Noop,
    #[serde(rename = "optin")]
    Optin,
    #[serde(rename = "closeout")]
    Closeout,
    #[serde(rename = "clear")]
    Clear,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "delete")]
    Delete,
}

impl Default for OnCompletion {
    fn default() -> Self {
        Self::Noop
    }
}

impl OnCompletion {
    pub fn to_u64(&self) -> u64 {
        match self {
            Self::Noop => 0,
            Self::Optin => 1,
            Self::Closeout => 2,
            Self::Clear => 3,
            Self::Update => 4,
            Self::Delete => 5,
        }
    }

    pub fn from_u64(num: u64) -> Result<Self> {
        match num {
            0 => Ok(Self::Noop),
            1 => Ok(Self::Optin),
            2 => Ok(Self::Closeout),
            3 => Ok(Self::Clear),
            4 => Ok(Self::Update),
            5 => Ok(Self::Delete),
            _ => Err(format!("Unrecognized u64 '{}' for `OnCompletion`!", num).into()),
        }
    }
}

impl Display for OnCompletion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Noop => "noop",
            Self::Optin => "optin",
            Self::Closeout => "closeout",
            Self::Clear => "clear",
            Self::Update => "update",
            Self::Delete => "delete",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for OnCompletion {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "noop" => Ok(Self::Noop),
            "optin" => Ok(Self::Optin),
            "clear" => Ok(Self::Clear),
            "delete" => Ok(Self::Delete),
            "update" => Ok(Self::Update),
            "closeout" => Ok(Self::Closeout),
            _ => Err(format!("Unrecognised `OnCompletion` fxn: {}!", s).into()),
        }
    }
}

/// Represents a `apls` local-state or `apgs` global-state schema. These schemas determine how
/// much storage may be used in a local-state or global-state for an application. The more space
/// used, the larger minimum balance must be maintained in the account holding the data.
#[derive(Clone, Debug, PartialEq, Default, Eq, Serialize, Deserialize)]
pub struct StateSchema {
    /// Maximum number of TEAL byte slices that may be stored in the key/value store.
    #[serde(rename = "num-byte-slice")]
    pub num_byte_slice: Option<u64>,

    /// Maximum number of TEAL uints that may be stored in the key/value store.
    #[serde(rename = "num-uint")]
    pub num_uint: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplicationTransactionJson {
    /// approval-program and clear-state-program.
    pub accounts: Option<Vec<String>>,

    /// `apaa` transaction specific arguments accessed from the application's approval-program and
    /// clear-state-program.
    #[serde(rename = "application-args")]
    pub application_args: Option<Vec<String>>,

    /// `apid` ID of the application being configured or empty if creating.
    #[serde(rename = "application-id")]
    pub application_id: Option<u64>,

    /// `apap` Logic executed for every application transaction, except when on-completion is set
    /// to "clear". It can read and write global state for the application, as well as
    /// account-specific local state. Approval programs may reject the transaction.
    ///
    /// Pattern : "^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==\|[A-Za-z0-9+/]{3}=)?$"
    #[serde(rename = "approval-program")]
    pub approval_program: Option<String>,

    /// `apsu` Logic executed for application transactions with on-completion set to "clear".
    /// It can read and write global state for the application, as well as account-specific local
    /// state. Clear state programs cannot reject the transaction.
    ///
    /// Pattern : "^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==\|[A-Za-z0-9+/]{3}=)?$"
    #[serde(rename = "clear-state-program")]
    pub clear_state_program: Option<String>,

    /// `apfa` Lists the applications in addition to the application-id whose global states may be
    /// accessed by this application's approval-program and clear-state-program. The access is
    /// read-only.
    #[serde(rename = "foreign-apps")]
    pub foreign_apps: Option<Vec<u64>>,

    /// `apas` lists the assets whose parameters may be accessed by this application's
    /// ApprovalProgram and ClearStateProgram. The access is read-only.
    #[serde(rename = "foreign-assets")]
    pub foreign_assets: Option<Vec<u64>>,

    /// Global state schema.
    #[serde(rename = "global-state-schema")]
    pub global_state_schema: Option<StateSchema>,

    /// Local state schema.
    #[serde(rename = "local-state-schema")]
    pub local_state_schema: Option<StateSchema>,

    /// On completion.
    #[serde(rename = "on-completion")]
    pub on_completion: Option<OnCompletion>,
}

impl FromStr for ApplicationTransactionJson {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}

impl ApplicationTransactionJson {
    pub fn is_empty(&self) -> bool {
        self.accounts.is_none()
            && self.application_args.is_none()
            && self.application_id.is_none()
            && self.approval_program.is_none()
            && self.clear_state_program.is_none()
            && self.foreign_apps.is_none()
            && self.foreign_assets.is_none()
            && self.global_state_schema.is_none()
            && self.local_state_schema.is_none()
            && self.on_completion.is_none()
    }

    pub fn maybe_get_application_args(&self) -> Result<Vec<AlgorandApplicationArg>> {
        match &self.application_args {
            None => Ok(vec![]),
            Some(encoded_strs) => encoded_strs
                .iter()
                .map(|encoded_str| Ok(AlgorandApplicationArg(base64_decode(encoded_str)?)))
                .collect(),
        }
    }

    pub fn maybe_get_accounts(&self) -> Result<Vec<AlgorandAddress>> {
        match &self.accounts {
            None => Ok(vec![]),
            Some(json_value) => json_value
                .iter()
                .map(|address| AlgorandAddress::from_str(address))
                .collect(),
        }
    }
}

impl AlgorandTransaction {
    /// ## Application Opt In
    ///
    /// Before an account can call a specific application it must opt-in.
    pub fn application_opt_in(
        application_id: u64,
        fee: &MicroAlgos,
        first_valid_round: u64,
        sender: &AlgorandAddress,
        genesis_hash: &AlgorandHash,
        last_valid_round: Option<u64>,
    ) -> Result<AlgorandTransaction> {
        Ok(Self {
            sender: Some(*sender),
            genesis_hash: Some(*genesis_hash),
            application_id: Some(application_id),
            on_completion: Some(OnCompletion::Optin.to_u64()),
            first_valid_round: Some(first_valid_round),
            fee: Some(fee.check_if_satisfies_minimum_fee()?.0),
            txn_type: Some(AlgorandTransactionType::ApplicationCall),
            last_valid_round: Some(Self::calculate_last_valid_round(
                first_valid_round,
                last_valid_round,
            )?),
            ..Default::default()
        })
    }

    /// ## Application Noop call
    ///
    /// A Noop application call is the one permitting to effectively execute the smart contract
    /// logic.
    /// * `application_id` - The application ID of the smart contract
    /// * `application_args` - The arguments to be passed to the smart contract.
    /// * `accounts` - A vector containing the accounts whose state will be accessed by the smart
    ///   contract
    /// * `foreign_apps` - A vector containing the application the smart contract may interact with
    /// * `foreign_assets` - A vector containing the ASAs the smart contract may retrieve
    ///   information from
    pub fn application_call_noop(
        application_id: u64,
        fee: MicroAlgos,
        first_valid_round: u64,
        sender: AlgorandAddress,
        genesis_hash: AlgorandHash,
        last_valid_round: Option<u64>,
        application_args: Option<Vec<AlgorandApplicationArg>>,
        accounts: Option<Vec<AlgorandAddress>>,
        foreign_apps: Option<Vec<u64>>,
        foreign_assets: Option<Vec<u64>>,
    ) -> Result<AlgorandTransaction> {
        Ok(Self {
            sender: Some(sender),
            genesis_hash: Some(genesis_hash),
            application_id: Some(application_id),
            first_valid_round: Some(first_valid_round),
            fee: Some(fee.check_if_satisfies_minimum_fee()?.0),
            txn_type: Some(AlgorandTransactionType::ApplicationCall),
            last_valid_round: Some(Self::calculate_last_valid_round(
                first_valid_round,
                last_valid_round,
            )?),
            application_args,
            accounts,
            foreign_apps,
            foreign_assets,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::{
        algorand_hash::AlgorandHash,
        test_utils::{get_sample_algorand_address, get_sample_algorand_keys},
    };

    #[test]
    fn should_sign_opt_in_transaction() {
        let tx = AlgorandTransaction::application_opt_in(
            90556484,
            &MicroAlgos(1000),
            21_682_035,
            &get_sample_algorand_address(),
            &AlgorandHash::testnet_genesis_hash().unwrap(),
            None,
        )
        .unwrap();
        let result = tx
            .sign(&get_sample_algorand_keys())
            .unwrap()
            .to_hex()
            .unwrap();
        let expected_result = "82a3736967c4407c383cc1e6e10b4f3bf44a9b0f0a56859ded08bdcc0bf096701052cb44d78062356542289d420ffc899c2b1f3acf165a22e032dd0b50f612a684efe21b53600ca374786e88a46170616e01a461706964ce0565c844a3666565cd03e8a26676ce014ad773a26768c4204863b518a4b3c84ec810f22d4f1081cb0f71f059a7ac20dec62f7f70e5093a22a26c76ce014adb5ba3736e64c42090826960db089ee5636266600d56a9f41f5d037e5c90a18007e384fc1558603da474797065a46170706c";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_sign_noop_call_transaction_with_no_args() {
        let tx = AlgorandTransaction::application_call_noop(
            90556484,
            MicroAlgos(1000),
            21_682_035,
            get_sample_algorand_address(),
            AlgorandHash::testnet_genesis_hash().unwrap(),
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        let result = tx
            .sign(&get_sample_algorand_keys())
            .unwrap()
            .to_hex()
            .unwrap();
        let expected_result = "82a3736967c440e511c544045ea0f33d8e54d343416bc4c8426aebb69a250e64b3dbfd97cc8d7c0a652b120ca162c866a3703f07b68f3cbb2d9543b46e28a2bf4bbb36894dfe08a374786e87a461706964ce0565c844a3666565cd03e8a26676ce014ad773a26768c4204863b518a4b3c84ec810f22d4f1081cb0f71f059a7ac20dec62f7f70e5093a22a26c76ce014adb5ba3736e64c42090826960db089ee5636266600d56a9f41f5d037e5c90a18007e384fc1558603da474797065a46170706c";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_sign_noop_call_transaction_with_args() {
        let mut args: Vec<AlgorandApplicationArg> = Vec::new();
        let arg1: &str = "2022-05-20T12:53:59.000Z";
        let arg2: i64 = 1234567890;
        args.push(AlgorandApplicationArg::from(arg1));
        args.push(AlgorandApplicationArg::from(arg2));
        let tx = AlgorandTransaction::application_call_noop(
            90556484,
            MicroAlgos(1000),
            21_682_035,
            get_sample_algorand_address(),
            AlgorandHash::testnet_genesis_hash().unwrap(),
            None,
            Some(args),
            None,
            None,
            None,
        )
        .unwrap();
        let result = tx
            .sign(&get_sample_algorand_keys())
            .unwrap()
            .to_hex()
            .unwrap();
        let expected_result = "82a3736967c440b67fa830b02182e7b771d91f9a782b91ea5c414f35be54ba7725446e42a6fdd97165edda069364e2fa5f5851ba29be21ab38ac0985c027a25f9204729dfcd006a374786e88a46170616192c418323032322d30352d32305431323a35333a35392e3030305ac40800000000499602d2a461706964ce0565c844a3666565cd03e8a26676ce014ad773a26768c4204863b518a4b3c84ec810f22d4f1081cb0f71f059a7ac20dec62f7f70e5093a22a26c76ce014adb5ba3736e64c42090826960db089ee5636266600d56a9f41f5d037e5c90a18007e384fc1558603da474797065a46170706c";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_sign_noop_call_transaction_with_empty_args() {
        let args: Vec<AlgorandApplicationArg> = Vec::new();
        let tx = AlgorandTransaction::application_call_noop(
            90556484,
            MicroAlgos(1000),
            21_682_035,
            get_sample_algorand_address(),
            AlgorandHash::testnet_genesis_hash().unwrap(),
            None,
            Some(args),
            None,
            None,
            None,
        )
        .unwrap();
        let result = tx
            .sign(&get_sample_algorand_keys())
            .unwrap()
            .to_hex()
            .unwrap();
        let expected_result = "82a3736967c440e511c544045ea0f33d8e54d343416bc4c8426aebb69a250e64b3dbfd97cc8d7c0a652b120ca162c866a3703f07b68f3cbb2d9543b46e28a2bf4bbb36894dfe08a374786e87a461706964ce0565c844a3666565cd03e8a26676ce014ad773a26768c4204863b518a4b3c84ec810f22d4f1081cb0f71f059a7ac20dec62f7f70e5093a22a26c76ce014adb5ba3736e64c42090826960db089ee5636266600d56a9f41f5d037e5c90a18007e384fc1558603da474797065a46170706c";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_sign_noop_call_transaction_with_args_and_accounts() {
        let mut args: Vec<AlgorandApplicationArg> = Vec::new();
        let arg1: &str = "2022-05-20T12:53:59.000Z";
        let arg2: i64 = 1234567890;
        args.push(AlgorandApplicationArg::from(arg1));
        args.push(AlgorandApplicationArg::from(arg2));
        let mut accounts: Vec<AlgorandAddress> = Vec::new();
        accounts.push(
            AlgorandAddress::from_str("GKT5XX6N45UV3ENMIOAVF7EQQYL77P45XFHYIPBFAJUON7RBUCQPX572TI")
                .unwrap(),
        );
        accounts.push(
            AlgorandAddress::from_str("YOR5IOP7NRQTM6QVYTJIOL76XLE2NR5AHQTTQEV4MTPCM4TLO3KTHY24RU")
                .unwrap(),
        );
        let tx = AlgorandTransaction::application_call_noop(
            90556484,
            MicroAlgos(1000),
            21_682_035,
            get_sample_algorand_address(),
            AlgorandHash::testnet_genesis_hash().unwrap(),
            None,
            Some(args),
            Some(accounts),
            None,
            None,
        )
        .unwrap();
        let result = tx
            .sign(&get_sample_algorand_keys())
            .unwrap()
            .to_hex()
            .unwrap();
        let expected_result = "82a3736967c440b053e99f1bc27c61cb15e19131026d2e81cad3a23e25f13f5066bf5db6c59d900c8ba263889cbc65d088d28d11c448dbb79e507844c51367e1f0d676b5993609a374786e89a46170616192c418323032322d30352d32305431323a35333a35392e3030305ac40800000000499602d2a46170617492c42032a7dbdfcde7695d91ac438152fc908617ffbf9db94f843c250268e6fe21a0a0c420c3a3d439ff6c61367a15c4d2872ffebac9a6c7a03c273812bc64de26726b76d5a461706964ce0565c844a3666565cd03e8a26676ce014ad773a26768c4204863b518a4b3c84ec810f22d4f1081cb0f71f059a7ac20dec62f7f70e5093a22a26c76ce014adb5ba3736e64c42090826960db089ee5636266600d56a9f41f5d037e5c90a18007e384fc1558603da474797065a46170706c";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_sign_noop_call_transaction_with_args_and_empty_accounts() {
        let mut args: Vec<AlgorandApplicationArg> = Vec::new();
        let arg1: &str = "2022-05-20T12:53:59.000Z";
        let arg2: i64 = 1234567890;
        args.push(AlgorandApplicationArg::from(arg1));
        args.push(AlgorandApplicationArg::from(arg2));
        let accounts: Vec<AlgorandAddress> = Vec::new();
        let tx = AlgorandTransaction::application_call_noop(
            90556484,
            MicroAlgos(1000),
            21_682_035,
            get_sample_algorand_address(),
            AlgorandHash::testnet_genesis_hash().unwrap(),
            None,
            Some(args),
            Some(accounts),
            None,
            None,
        )
        .unwrap();
        let result = tx
            .sign(&get_sample_algorand_keys())
            .unwrap()
            .to_hex()
            .unwrap();
        let expected_result = "82a3736967c440b67fa830b02182e7b771d91f9a782b91ea5c414f35be54ba7725446e42a6fdd97165edda069364e2fa5f5851ba29be21ab38ac0985c027a25f9204729dfcd006a374786e88a46170616192c418323032322d30352d32305431323a35333a35392e3030305ac40800000000499602d2a461706964ce0565c844a3666565cd03e8a26676ce014ad773a26768c4204863b518a4b3c84ec810f22d4f1081cb0f71f059a7ac20dec62f7f70e5093a22a26c76ce014adb5ba3736e64c42090826960db089ee5636266600d56a9f41f5d037e5c90a18007e384fc1558603da474797065a46170706c";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_sign_noop_call_transaction_with_args_and_accounts_and_foreign_apps() {
        let mut args: Vec<AlgorandApplicationArg> = Vec::new();
        let arg1: &str = "2022-05-20T12:53:59.000Z";
        let arg2: i64 = 1234567890;
        args.push(AlgorandApplicationArg::from(arg1));
        args.push(AlgorandApplicationArg::from(arg2));
        let mut accounts: Vec<AlgorandAddress> = Vec::new();
        accounts.push(
            AlgorandAddress::from_str("GKT5XX6N45UV3ENMIOAVF7EQQYL77P45XFHYIPBFAJUON7RBUCQPX572TI")
                .unwrap(),
        );
        accounts.push(
            AlgorandAddress::from_str("YOR5IOP7NRQTM6QVYTJIOL76XLE2NR5AHQTTQEV4MTPCM4TLO3KTHY24RU")
                .unwrap(),
        );
        let mut foreign_apps: Vec<u64> = Vec::new();
        foreign_apps.push(123456789);
        foreign_apps.push(987654321);
        let tx = AlgorandTransaction::application_call_noop(
            90556484,
            MicroAlgos(1000),
            21_682_035,
            get_sample_algorand_address(),
            AlgorandHash::testnet_genesis_hash().unwrap(),
            None,
            Some(args),
            Some(accounts),
            Some(foreign_apps),
            None,
        )
        .unwrap();
        let result = tx
            .sign(&get_sample_algorand_keys())
            .unwrap()
            .to_hex()
            .unwrap();
        let expected_result = "82a3736967c44085b35ccbdf3da244c1be78b7b72777365bb73620ede91a9503b1d3a8e1795228097dadef938d410441b18cde59223a34dcafa6d040fb8cb8dbb2f3e8a2b21f0ba374786e8aa46170616192c418323032322d30352d32305431323a35333a35392e3030305ac40800000000499602d2a46170617492c42032a7dbdfcde7695d91ac438152fc908617ffbf9db94f843c250268e6fe21a0a0c420c3a3d439ff6c61367a15c4d2872ffebac9a6c7a03c273812bc64de26726b76d5a46170666192ce075bcd15ce3ade68b1a461706964ce0565c844a3666565cd03e8a26676ce014ad773a26768c4204863b518a4b3c84ec810f22d4f1081cb0f71f059a7ac20dec62f7f70e5093a22a26c76ce014adb5ba3736e64c42090826960db089ee5636266600d56a9f41f5d037e5c90a18007e384fc1558603da474797065a46170706c";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_sign_noop_call_transaction_with_args_and_accounts_and_empty_foreign_apps() {
        let mut args: Vec<AlgorandApplicationArg> = Vec::new();
        let arg1: &str = "2022-05-20T12:53:59.000Z";
        let arg2: i64 = 1234567890;
        args.push(AlgorandApplicationArg::from(arg1));
        args.push(AlgorandApplicationArg::from(arg2));
        let mut accounts: Vec<AlgorandAddress> = Vec::new();
        accounts.push(
            AlgorandAddress::from_str("GKT5XX6N45UV3ENMIOAVF7EQQYL77P45XFHYIPBFAJUON7RBUCQPX572TI")
                .unwrap(),
        );
        accounts.push(
            AlgorandAddress::from_str("YOR5IOP7NRQTM6QVYTJIOL76XLE2NR5AHQTTQEV4MTPCM4TLO3KTHY24RU")
                .unwrap(),
        );
        let foreign_apps: Vec<u64> = Vec::new();
        let tx = AlgorandTransaction::application_call_noop(
            90556484,
            MicroAlgos(1000),
            21_682_035,
            get_sample_algorand_address(),
            AlgorandHash::testnet_genesis_hash().unwrap(),
            None,
            Some(args),
            Some(accounts),
            Some(foreign_apps),
            None,
        )
        .unwrap();
        let result = tx
            .sign(&get_sample_algorand_keys())
            .unwrap()
            .to_hex()
            .unwrap();
        let expected_result = "82a3736967c440b053e99f1bc27c61cb15e19131026d2e81cad3a23e25f13f5066bf5db6c59d900c8ba263889cbc65d088d28d11c448dbb79e507844c51367e1f0d676b5993609a374786e89a46170616192c418323032322d30352d32305431323a35333a35392e3030305ac40800000000499602d2a46170617492c42032a7dbdfcde7695d91ac438152fc908617ffbf9db94f843c250268e6fe21a0a0c420c3a3d439ff6c61367a15c4d2872ffebac9a6c7a03c273812bc64de26726b76d5a461706964ce0565c844a3666565cd03e8a26676ce014ad773a26768c4204863b518a4b3c84ec810f22d4f1081cb0f71f059a7ac20dec62f7f70e5093a22a26c76ce014adb5ba3736e64c42090826960db089ee5636266600d56a9f41f5d037e5c90a18007e384fc1558603da474797065a46170706c";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_sign_noop_call_transaction_with_args_and_accounts_and_foreign_apps_and_foreign_assets(
    ) {
        let mut args: Vec<AlgorandApplicationArg> = Vec::new();
        let arg1: &str = "2022-05-20T12:53:59.000Z";
        let arg2: i64 = 1234567890;
        args.push(AlgorandApplicationArg::from(arg1));
        args.push(AlgorandApplicationArg::from(arg2));
        let mut accounts: Vec<AlgorandAddress> = Vec::new();
        accounts.push(
            AlgorandAddress::from_str("GKT5XX6N45UV3ENMIOAVF7EQQYL77P45XFHYIPBFAJUON7RBUCQPX572TI")
                .unwrap(),
        );
        accounts.push(
            AlgorandAddress::from_str("YOR5IOP7NRQTM6QVYTJIOL76XLE2NR5AHQTTQEV4MTPCM4TLO3KTHY24RU")
                .unwrap(),
        );
        let mut foreign_apps: Vec<u64> = Vec::new();
        foreign_apps.push(123456789);
        let mut foreign_assets: Vec<u64> = Vec::new();
        foreign_assets.push(12345);
        foreign_assets.push(67890);
        let tx = AlgorandTransaction::application_call_noop(
            90556484,
            MicroAlgos(1000),
            21_682_035,
            get_sample_algorand_address(),
            AlgorandHash::testnet_genesis_hash().unwrap(),
            None,
            Some(args),
            Some(accounts),
            Some(foreign_apps),
            Some(foreign_assets),
        )
        .unwrap();
        let result = tx
            .sign(&get_sample_algorand_keys())
            .unwrap()
            .to_hex()
            .unwrap();
        let expected_result = "82a3736967c440827057e4529b51e18f560ed293b8139e7b0f9b7ff1470c6c48adf5d038a51764508a76040693e3f7fe44d0ce82c2c9492a45bde186f6081ba86cf07c39276b02a374786e8ba46170616192c418323032322d30352d32305431323a35333a35392e3030305ac40800000000499602d2a46170617392cd3039ce00010932a46170617492c42032a7dbdfcde7695d91ac438152fc908617ffbf9db94f843c250268e6fe21a0a0c420c3a3d439ff6c61367a15c4d2872ffebac9a6c7a03c273812bc64de26726b76d5a46170666191ce075bcd15a461706964ce0565c844a3666565cd03e8a26676ce014ad773a26768c4204863b518a4b3c84ec810f22d4f1081cb0f71f059a7ac20dec62f7f70e5093a22a26c76ce014adb5ba3736e64c42090826960db089ee5636266600d56a9f41f5d037e5c90a18007e384fc1558603da474797065a46170706c";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_sign_noop_call_transaction_with_args_and_accounts_and_foreign_apps_and_empty_foreign_assets(
    ) {
        let mut args: Vec<AlgorandApplicationArg> = Vec::new();
        let arg1: &str = "2022-05-20T12:53:59.000Z";
        let arg2: i64 = 1234567890;
        args.push(AlgorandApplicationArg::from(arg1));
        args.push(AlgorandApplicationArg::from(arg2));
        let mut accounts: Vec<AlgorandAddress> = Vec::new();
        accounts.push(
            AlgorandAddress::from_str("GKT5XX6N45UV3ENMIOAVF7EQQYL77P45XFHYIPBFAJUON7RBUCQPX572TI")
                .unwrap(),
        );
        accounts.push(
            AlgorandAddress::from_str("YOR5IOP7NRQTM6QVYTJIOL76XLE2NR5AHQTTQEV4MTPCM4TLO3KTHY24RU")
                .unwrap(),
        );
        let mut foreign_apps: Vec<u64> = Vec::new();
        foreign_apps.push(123456789);
        let foreign_assets: Vec<u64> = Vec::new();
        let tx = AlgorandTransaction::application_call_noop(
            90556484,
            MicroAlgos(1000),
            21_682_035,
            get_sample_algorand_address(),
            AlgorandHash::testnet_genesis_hash().unwrap(),
            None,
            Some(args),
            Some(accounts),
            Some(foreign_apps),
            Some(foreign_assets),
        )
        .unwrap();
        let result = tx
            .sign(&get_sample_algorand_keys())
            .unwrap()
            .to_hex()
            .unwrap();
        let expected_result = "82a3736967c440d6c55d1cdaa0f7556506699f67c0730f82baf09fe2f06ecd175631a672aab5a8f12c037906a3358f3a482c6b29601971a72a8e608c818e1121abc3d701769001a374786e8aa46170616192c418323032322d30352d32305431323a35333a35392e3030305ac40800000000499602d2a46170617492c42032a7dbdfcde7695d91ac438152fc908617ffbf9db94f843c250268e6fe21a0a0c420c3a3d439ff6c61367a15c4d2872ffebac9a6c7a03c273812bc64de26726b76d5a46170666191ce075bcd15a461706964ce0565c844a3666565cd03e8a26676ce014ad773a26768c4204863b518a4b3c84ec810f22d4f1081cb0f71f059a7ac20dec62f7f70e5093a22a26c76ce014adb5ba3736e64c42090826960db089ee5636266600d56a9f41f5d037e5c90a18007e384fc1558603da474797065a46170706c";
        assert_eq!(result, expected_result);
    }
}
