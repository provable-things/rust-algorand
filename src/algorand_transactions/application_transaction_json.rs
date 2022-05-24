use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{algorand_errors::AlgorandError, algorand_types::Result};

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
    /// `apat` List of accounts in addition to the sender that may be accessed from the
    /// application's
    #[serde(rename = "noop")]
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
}
