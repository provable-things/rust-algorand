use crate::{
    algorand_address::AlgorandAddress,
    algorand_hash::AlgorandHash,
    algorand_keys::AlgorandKeys,
    algorand_micro_algos::MicroAlgos,
    algorand_signature::AlgorandSignature,
    algorand_traits::ToMsgPackBytes,
    algorand_transaction::AlgorandTransaction,
    algorand_types::{Byte, Bytes, Result},
    constants::ALGORAND_MAX_NUM_ROUNDS,
    crypto_utils::{base32_encode_with_no_padding, sha512_256_hash_bytes},
};

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        algorand_transaction::transaction_test_utils::get_sample_pay_tx,
        constants::ALGORAND_MAINNET_GENESIS_ID,
        errors::AppError,
        test_utils::get_sample_algorand_keys,
    };

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

    #[test]
    fn should_encode_tx_to_msg_pack_bytes() {
        let tx = get_sample_pay_tx();
        let result = hex::encode(tx.to_msg_pack_bytes().unwrap());
        let expected_result = "88a3616d74cd0539a3666565cd03e8a26676cd03e8a26768c420c061c4d8fc1dbdded2d7604be4568e3f6d041987ac37bde4b620b5ab39248adfa26c76cd07d0a3726376c4203516382099d0a8f0c8eabc6d4b7efb58b0be7008e98d239dd0199490674e0372a3736e64c420e23319d1dfd271db1f8752ea384948a975594617fa0b546d44740e768a14b899a474797065a3706179";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_encode_tx_for_signing() {
        let tx = get_sample_pay_tx();
        let result = hex::encode(tx.encode_for_signing().unwrap());
        let expected_result = "545888a3616d74cd0539a3666565cd03e8a26676cd03e8a26768c420c061c4d8fc1dbdded2d7604be4568e3f6d041987ac37bde4b620b5ab39248adfa26c76cd07d0a3726376c4203516382099d0a8f0c8eabc6d4b7efb58b0be7008e98d239dd0199490674e0372a3736e64c420e23319d1dfd271db1f8752ea384948a975594617fa0b546d44740e768a14b899a474797065a3706179";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_get_transaction_hash() {
        let tx = get_sample_pay_tx();
        let result = tx.to_id().unwrap();
        let expected_result = "CT3DHZ5ZK6VZWXIMDZXAVIVH6DA4V26HWBPCHIZGHLVWFEOC7P6A";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_sign_tx() {
        let tx = get_sample_pay_tx();
        let keys = get_sample_algorand_keys();
        let signed_tx = tx.sign(&keys).unwrap();
        let result = hex::encode(signed_tx.to_msg_pack_bytes().unwrap());
        let expected_result = "82a3736967c440a6e1b839f2f0109afa914694264e1fcee2ce3b5858ff46c436a69ece9f9630d9d182b1307f4bb8a1807e785326f91beded1a6d1a2368c1e0644b58bf6bc60f06a374786e88a3616d74cd0539a3666565cd03e8a26676cd03e8a26768c420c061c4d8fc1dbdded2d7604be4568e3f6d041987ac37bde4b620b5ab39248adfa26c76cd07d0a3726376c4203516382099d0a8f0c8eabc6d4b7efb58b0be7008e98d239dd0199490674e0372a3736e64c420e23319d1dfd271db1f8752ea384948a975594617fa0b546d44740e768a14b899a474797065a3706179";
        assert_eq!(result, expected_result);
    }
}
