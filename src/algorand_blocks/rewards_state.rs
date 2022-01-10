use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json;
use serde_with::skip_serializing_none;

use crate::{algorand_address::AlgorandAddress, algorand_types::Result, errors::AppError};

/// Represents the global parameters controlling the rate at which accounts accrue rewards.
#[skip_serializing_none]
#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RewardsState {
    /// Accepts transaction fees.
    #[serde(rename = "fees")]
    fee_sink: AlgorandAddress,

    /// Accepts periodic injections from `fees` and continually redistributes them to addresses as
    /// rewards.
    #[serde(rename = "rwd")]
    rewards_pool: AlgorandAddress,

    /// Specifies how many rewards, in MicroAlgos, have been distributed to each
    /// config.Protocol.RewardUnit of MicroAlgos since genesis.
    #[serde(rename = "earn", default)]
    rewards_level: Option<u64>,

    /// The number of new MicroAlgos added to the participation stake from rewards at the next
    /// round.
    #[serde(rename = "rate", default)]
    rewards_rate: Option<u64>,

    /// The number of leftover MicroAlgos after rewards distribution.
    #[serde(rename = "frac", default)]
    rewards_residue: Option<u64>,

    /// The round at which the RewardsRate will be recalculated.
    #[serde(rename = "rwcalr")]
    rewards_calculation_round: u64,
}

impl RewardsState {
    pub fn from_json(json: &RewardsStateJson) -> Result<Self> {
        json.to_rewards_state()
    }
}

impl FromStr for RewardsState {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self> {
        RewardsStateJson::from_str(s).and_then(|json| json.to_rewards_state())
    }
}

/// Represents the global parameters controlling the rate at which accounts accrue rewards.
#[skip_serializing_none]
#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RewardsStateJson {
    #[serde(rename = "fee-sink")]
    pub fee_sink: String,

    #[serde(rename = "rewards-pool")]
    pub rewards_pool: String,

    #[serde(rename = "rewards-level", default)]
    pub rewards_level: Option<u64>,

    #[serde(rename = "rewards-rate", default)]
    pub rewards_rate: Option<u64>,

    #[serde(rename = "rewards-residue", default)]
    pub rewards_residue: Option<u64>,

    #[serde(rename = "rewards-calculation-round")]
    pub rewards_calculation_round: u64,
}

impl RewardsStateJson {
    fn to_rewards_state(&self) -> Result<RewardsState> {
        Ok(RewardsState {
            rewards_rate: self.rewards_rate,
            rewards_level: self.rewards_level,
            rewards_residue: self.rewards_residue,
            fee_sink: AlgorandAddress::from_str(&self.fee_sink)?,
            rewards_calculation_round: self.rewards_calculation_round,
            rewards_pool: AlgorandAddress::from_str(&self.rewards_pool)?,
        })
    }
}

impl FromStr for RewardsStateJson {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}
