use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use serde_with::skip_serializing_none;

use crate::{
    algorand_address::AlgorandAddress,
    algorand_errors::AlgorandError,
    algorand_hash::AlgorandHash,
    algorand_types::Result,
};

#[skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AssetParameters {
    #[serde(rename(serialize = "am"))]
    pub metadata_hash: Option<AlgorandHash>,

    #[serde(rename(serialize = "an"))]
    pub asset_name: Option<String>,

    #[serde(rename(serialize = "au"))]
    pub asset_url: Option<String>,

    /// ## Clawback Address
    ///
    /// The clawback address represents an account that is allowed to transfer assets from and to
    /// any asset holder (assuming they have opted-in). Use this if you need the option to revoke
    /// assets from an account (like if they breach certain contractual obligations tied to holding
    /// the asset). In traditional finance, this sort of transaction is referred to as a clawback.
    #[serde(rename(serialize = "c"))]
    pub clawback_address: Option<AlgorandAddress>,

    #[serde(rename(serialize = "dc"))]
    pub decimals: u64,

    /// ## Default Frozen
    ///
    /// Whether the asset is created in a froze state.
    #[serde(rename(serialize = "df"))]
    pub default_frozen: Option<bool>,

    /// ## Freeze Address
    ///
    /// The freeze account is allowed to freeze or unfreeze the asset holdings for a specific
    /// account. When an account is frozen it cannot send or receive the frozen asset. In
    /// traditional finance, freezing assets may be performed to restrict liquidation of company
    /// stock, to investigate suspected criminal activity or to deny-list certain accounts. If the
    /// DefaultFrozen state is set to True, you can use the unfreeze action to authorize certain
    /// accounts to trade the asset (such as after passing KYC/AML checks).
    #[serde(rename(serialize = "f"))]
    pub freeze_address: Option<AlgorandAddress>,

    /// ## Manager Address
    ///
    /// The manager account is the only account that can authorize transactions to re-configure or
    /// destroy an asset.
    #[serde(rename(serialize = "m"))]
    pub manager_address: Option<AlgorandAddress>,

    /// ## Reserve Address
    ///
    /// Specifying a reserve account signifies that non-minted assets will reside in that account
    /// instead of the default creator account. Assets transferred from this account are "minted"
    /// units of the asset. If you specify a new reserve address, you must make sure the new
    /// account has opted into the asset and then issue a transaction to transfer all assets to the
    /// new reserve.
    #[serde(rename(serialize = "r"))]
    pub reserve_address: Option<AlgorandAddress>,

    #[serde(rename(serialize = "t"))]
    pub total_base_units: u64,

    #[serde(rename(serialize = "un"))]
    pub unit_name: Option<String>,
}

impl AssetParameters {
    pub fn new(
        metadata_hash: Option<AlgorandHash>,
        asset_name: Option<String>,
        asset_url: Option<String>,
        clawback_address: Option<AlgorandAddress>,
        decimals: u64,
        default_frozen: bool,
        freeze_address: Option<AlgorandAddress>,
        manager_address: Option<AlgorandAddress>,
        reserve_address: Option<AlgorandAddress>,
        total_base_units: u64,
        unit_name: Option<String>,
    ) -> Self {
        Self {
            decimals,
            unit_name,
            asset_name,
            metadata_hash,
            freeze_address,
            manager_address,
            reserve_address,
            clawback_address,
            total_base_units,
            asset_url,
            default_frozen: if default_frozen { Some(true) } else { None },
        }
    }

    pub fn from_json(json: &AssetParametersJson) -> Result<Self> {
        Ok(Self {
            decimals: json.decimals,
            unit_name: json.unit_name.clone(),
            asset_url: json.asset_url.clone(),
            asset_name: json.asset_name.clone(),
            default_frozen: json.default_frozen,
            total_base_units: if json.total_base_units.is_u64() {
                json.total_base_units.as_u64().expect("Should never fail")
            } else {
                u64::MAX
            },
            freeze_address: match &json.freeze_address {
                Some(address_str) => Some(AlgorandAddress::from_str(address_str)?),
                None => None,
            },
            manager_address: match &json.manager_address {
                Some(address_str) => Some(AlgorandAddress::from_str(address_str)?),
                None => None,
            },
            reserve_address: match &json.reserve_address {
                Some(address_str) => Some(AlgorandAddress::from_str(address_str)?),
                None => None,
            },
            clawback_address: match &json.clawback_address {
                Some(address_str) => Some(AlgorandAddress::from_str(address_str)?),
                None => None,
            },
            metadata_hash: match &json.metadata_hash {
                Some(hash_str) => Some(AlgorandHash::from_str(hash_str)?),
                None => None,
            },
        })
    }

    pub fn to_json(&self) -> Result<AssetParametersJson> {
        Ok(AssetParametersJson {
            decimals: self.decimals,
            unit_name: self.unit_name.clone(),
            asset_url: self.asset_url.clone(),
            asset_name: self.asset_name.clone(),
            default_frozen: self.default_frozen,
            total_base_units: json!(self.total_base_units),
            metadata_hash: self.metadata_hash.as_ref().map(|x| x.to_string()),
            freeze_address: self.freeze_address.as_ref().map(|x| x.to_string()),
            reserve_address: self.reserve_address.as_ref().map(|x| x.to_string()),
            manager_address: self.manager_address.as_ref().map(|x| x.to_string()),
            clawback_address: self.clawback_address.as_ref().map(|x| x.to_string()),
        })
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AssetParametersJson {
    #[serde(rename = "metadata-hash")]
    pub metadata_hash: Option<String>,

    #[serde(rename = "name")]
    pub asset_name: Option<String>,

    #[serde(rename = "unit")]
    pub unit_name: Option<String>,

    #[serde(rename = "url")]
    pub asset_url: Option<String>,

    #[serde(rename = "clawback")]
    pub clawback_address: Option<String>,

    pub decimals: u64,

    #[serde(rename = "default-frozen")]
    pub default_frozen: Option<bool>,

    #[serde(rename = "freeze")]
    pub freeze_address: Option<String>,

    #[serde(rename = "manager")]
    pub manager_address: Option<String>,

    #[serde(rename = "reserve")]
    pub reserve_address: Option<String>,

    // NOTE: Some blocks exist with > u64::MAX as this field, but the node reports the
    // actual value as u64::MAX. Interesting!
    #[serde(rename = "total")]
    pub total_base_units: JsonValue,
}

impl FromStr for AssetParametersJson {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorand_transactions::test_utils::get_sample_asset_parameters_json_str;

    #[test]
    fn should_get_asset_parameters_from_str() {
        let result = AssetParametersJson::from_str(&get_sample_asset_parameters_json_str());
        assert!(result.is_ok());
    }

    #[test]
    fn should_get_asset_parameters_from_json() {
        let json = AssetParametersJson::from_str(&get_sample_asset_parameters_json_str()).unwrap();
        let result = AssetParameters::from_json(&json);
        assert!(result.is_ok());
    }
}
