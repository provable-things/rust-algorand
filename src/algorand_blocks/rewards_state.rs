use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    algorand_address::AlgorandAddress,
    algorand_errors::AlgorandError,
    algorand_types::Result,
};

/// Represents the global parameters controlling the rate at which accounts accrue rewards.
#[skip_serializing_none]
#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RewardsState {
    /// Accepts transaction fees.
    #[serde(rename = "fees")]
    pub fee_sink: Option<AlgorandAddress>,

    /// Accepts periodic injections from `fees` and continually redistributes them to addresses as
    /// rewards.
    #[serde(rename = "rwd")]
    pub rewards_pool: Option<AlgorandAddress>,

    /// Specifies how many rewards, in MicroAlgos, have been distributed to each
    /// config.Protocol.RewardUnit of MicroAlgos since genesis.
    #[serde(rename = "earn", default)]
    pub rewards_level: Option<u64>,

    /// The number of new MicroAlgos added to the participation stake from rewards at the next
    /// round.
    #[serde(rename = "rate", default)]
    pub rewards_rate: Option<u64>,

    /// The number of leftover MicroAlgos after rewards distribution.
    #[serde(rename = "frac", default)]
    pub rewards_residue: Option<u64>,

    /// The round at which the RewardsRate will be recalculated.
    #[serde(rename = "rwcalr")]
    pub rewards_calculation_round: Option<u64>,
}

impl RewardsState {
    pub fn from_json(json: &RewardsStateJson) -> Result<Self> {
        json.to_rewards_state()
    }

    pub fn to_json(&self) -> Result<RewardsStateJson> {
        Ok(RewardsStateJson {
            rewards_rate: self.rewards_rate,
            rewards_level: self.rewards_level,
            rewards_residue: self.rewards_residue,
            fee_sink: self.fee_sink.as_ref().map(|x| x.to_string()),
            rewards_pool: self.rewards_pool.as_ref().map(|x| x.to_string()),
            rewards_calculation_round: self.rewards_calculation_round,
        })
    }
}

impl FromStr for RewardsState {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        RewardsStateJson::from_str(s).and_then(|json| json.to_rewards_state())
    }
}

// TODO move to own mod?
/// Represents the global parameters controlling the rate at which accounts accrue rewards.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RewardsStateJson {
    #[serde(rename = "fee-sink")]
    pub fee_sink: Option<String>,

    #[serde(rename = "rewards-pool")]
    pub rewards_pool: Option<String>,

    #[serde(rename = "rewards-level", default)]
    pub rewards_level: Option<u64>,

    #[serde(rename = "rewards-rate", default)]
    pub rewards_rate: Option<u64>,

    #[serde(rename = "rewards-residue", default)]
    pub rewards_residue: Option<u64>,

    #[serde(rename = "rewards-calculation-round")]
    pub rewards_calculation_round: Option<u64>,
}

impl RewardsStateJson {
    fn to_rewards_state(&self) -> Result<RewardsState> {
        Ok(RewardsState {
            rewards_rate: self.rewards_rate,
            rewards_level: self.rewards_level,
            rewards_residue: self.rewards_residue,
            rewards_calculation_round: self.rewards_calculation_round,
            fee_sink: match &self.fee_sink {
                Some(address_string) => Some(AlgorandAddress::from_str(address_string)?),
                None => None,
            },
            rewards_pool: match &self.rewards_pool {
                Some(address_string) => Some(AlgorandAddress::from_str(address_string)?),
                None => None,
            },
        })
    }

    pub fn is_empty(&self) -> bool {
        self.fee_sink.is_none()
            && self.rewards_rate.is_none()
            && self.rewards_pool.is_none()
            && self.rewards_level.is_none()
            && self.rewards_residue.is_none()
            && self.rewards_calculation_round.is_none()
    }
}

impl FromStr for RewardsStateJson {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}

// TODO Impl Display!

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn should_parse_rewards_state_from_json() {
        let json_string = json!({
          "fee-sink": "Y76M3MSY6DKBRHBL7C3NNDXGS5IIMQVQVUAB6MP4XEMMGVF2QWNPL226CA",
          "rewards-calculation-round": 18000000,
          "rewards-level": 214862,
          "rewards-pool": "737777777777777777777777777777777777777777777777777UFEJ2CI",
          "rewards-rate": 43700000,
          "rewards-residue": 812839965
        })
        .to_string();
        let result = RewardsStateJson::from_str(&json_string);
        assert!(result.is_ok());
    }
}
