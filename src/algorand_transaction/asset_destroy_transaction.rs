use crate::{
    algorand_address::AlgorandAddress,
    algorand_hash::AlgorandHash,
    algorand_micro_algos::MicroAlgos,
    algorand_transaction::{transaction_type::AlgorandTransactionType, AlgorandTransaction},
    algorand_types::Result,
};

impl AlgorandTransaction {
    /// ## New Asset Destroy Transaction
    ///
    /// A Destroy Transaction is issued to remove an asset from the Algorand ledger. To destroy an
    /// existing asset on Algorand, the original creator must be in possession of all units of the
    /// asset and the manager must send and therefore authorize the transaction.
    pub fn new_asset_destroy_tx(
        asset_id: u64,
        fee: MicroAlgos,
        first_valid_round: u64,
        sender: AlgorandAddress,
        genesis_hash: AlgorandHash,
        last_valid_round: Option<u64>,
    ) -> Result<Self> {
        let calculated_last_valid_round =
            Self::calculate_last_valid_round(first_valid_round, last_valid_round)?;
        Ok(Self {
            sender,
            genesis_hash,
            first_valid_round,
            asset_id: Some(asset_id),
            fee: fee.check_if_satisfies_minimum_fee()?.0,
            last_valid_round: calculated_last_valid_round,
            txn_type: AlgorandTransactionType::AssetConfiguration,
            note: None,
            group: None,
            lease: None,
            amount: None,
            rekey_to: None,
            receiver: None,
            genesis_id: None,
            asset_parameters: None,
            close_remainder_to: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{get_sample_algorand_address, get_sample_algorand_keys};

    #[test]
    fn should_sign_asset_destory_transaction() {
        let tx = AlgorandTransaction::new_asset_destroy_tx(
            460341519,
            MicroAlgos(1000),
            17958943,
            get_sample_algorand_address(),
            AlgorandHash::mainnet_genesis_hash().unwrap(),
            None,
        )
        .unwrap();
        let result = tx
            .sign(&get_sample_algorand_keys())
            .unwrap()
            .to_hex()
            .unwrap();
        let expected_result = "82a3736967c44001d5422690355726c695ccc155d6c1a31fa75e2d49dc7cabef22eba22c7b8e56fd532eb2a93cf558f4a176c20106ca24d01adae59851c320578d35910ddfa505a374786e87a463616964ce1b70410fa3666565cd03e8a26676ce0112081fa26768c420c061c4d8fc1dbdded2d7604be4568e3f6d041987ac37bde4b620b5ab39248adfa26c76ce01120c07a3736e64c42090826960db089ee5636266600d56a9f41f5d037e5c90a18007e384fc1558603da474797065a461636667";
        assert_eq!(result, expected_result);
    }
}
