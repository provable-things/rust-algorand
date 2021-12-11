use std::{
    cmp::Ordering,
    fmt,
    ops::{Add, Mul, Sub},
};

use serde::{Serialize, Serializer};

use crate::algorand_types::Result;

const ALGORAND_MINIMUM_FEE: u64 = 1_000;
pub(crate) const MICRO_ALGOS_MULTIPLIER: u64 = 1_000_000;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct MicroAlgos(pub(crate) u64);

impl MicroAlgos {
    /// From Algos
    ///
    /// Create MicroAlgos from Algos.
    pub fn from_algos(algos: u64) -> Result<Self> {
        Ok(Self(algos / MICRO_ALGOS_MULTIPLIER))
    }

    fn to_algos(&self) -> u64 {
        self.0 * MICRO_ALGOS_MULTIPLIER
    }

    fn satisfies_minimum_fee(&self) -> bool {
        self >= &Self::minimum_fee()
    }

    /// ## Minimum Fee
    ///
    /// Get the minimum fee for an Algorand transaction in MicroAlgos.
    pub fn minimum_fee() -> Self {
        Self(ALGORAND_MINIMUM_FEE)
    }

    /// ## Check if Satisfies Minimum Fee
    ///
    /// ## Check is an amount of MicroAlgos satisfies the minimum transaction fee required by the
    /// Algorand protocol.
    pub fn check_if_satisfies_minimum_fee(&self) -> Result<Self> {
        if self.satisfies_minimum_fee() {
            Ok(self.clone())
        } else {
            Err(format!(
                "Fee is below the minimum algorand fee of {} MicroAlgos!",
                ALGORAND_MINIMUM_FEE
            )
            .into())
        }
    }
}

impl fmt::Display for MicroAlgos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_algos())
    }
}

impl Add<u64> for MicroAlgos {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        MicroAlgos(self.0 + rhs)
    }
}

impl Sub<u64> for MicroAlgos {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        MicroAlgos(self.0 - rhs)
    }
}

impl Mul<u64> for MicroAlgos {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        MicroAlgos(self.0 * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::AppError;

    #[test]
    fn minimum_fee_should_be_correct() {
        let result = MicroAlgos::minimum_fee().to_algos();
        let expected_result = ALGORAND_MINIMUM_FEE * MICRO_ALGOS_MULTIPLIER;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_satisfy_minimum_fee() {
        let num = ALGORAND_MINIMUM_FEE * MICRO_ALGOS_MULTIPLIER;
        let result = MicroAlgos(num).check_if_satisfies_minimum_fee();
        assert!(result.is_ok());
    }

    #[test]
    fn should_fail_minimum_fee_check_if_amount_too_low() {
        let num = (ALGORAND_MINIMUM_FEE * MICRO_ALGOS_MULTIPLIER) - 1;
        let expected_error = format!(
            "Fee is below the minimum algorand fee of {} MicroAlgos!",
            ALGORAND_MINIMUM_FEE
        );
        match MicroAlgos::from_algos(num)
            .unwrap()
            .check_if_satisfies_minimum_fee()
        {
            Ok(_) => panic!("Should not have succeeded!"),
            Err(AppError::Custom(error)) => assert_eq!(error, expected_error),
            Err(_) => panic!("Wrong err received!"),
        };
    }
}
