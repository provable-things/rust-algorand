use crate::{
    algorand_address::AlgorandAddress,
    algorand_hash::AlgorandHash,
    algorand_micro_algos::MicroAlgos,
    algorand_transactions::{
        transaction::AlgorandTransaction,
        transaction_type::AlgorandTransactionType,
    },
    algorand_types::Result,
};

impl AlgorandTransaction {
    /// ## Asset Transfer
    ///
    /// Assets can be transferred between accounts that have opted-in to receiving the asset. These
    /// are analogous to standard payment transactions but for Algorand Standard Assets.
    pub fn asset_transfer(
        asset_id: u64,
        fee: MicroAlgos,
        asset_amount: u64,
        first_valid_round: u64,
        sender: AlgorandAddress,
        genesis_hash: AlgorandHash,
        last_valid_round: Option<u64>,
        asset_receiver: AlgorandAddress,
    ) -> Result<AlgorandTransaction> {
        Ok(Self {
            sender,
            genesis_hash,
            first_valid_round,
            asset_amount: Some(asset_amount),
            transfer_asset_id: Some(asset_id),
            asset_receiver: Some(asset_receiver),
            fee: fee.check_if_satisfies_minimum_fee()?.0,
            last_valid_round: Self::calculate_last_valid_round(
                first_valid_round,
                last_valid_round,
            )?,
            txn_type: AlgorandTransactionType::AssetTransfer,
            note: None,
            group: None,
            lease: None,
            amount: None,
            asset_id: None,
            rekey_to: None,
            receiver: None,
            genesis_id: None,
            asset_sender: None,
            asset_parameters: None,
            close_remainder_to: None,
        })
    }

    /// Asset Opt In
    ///
    /// Before an account can receive a specific asset it must opt-in to receive it. An opt-in
    /// transaction places an asset holding of 0 into the account and increases its minimum balance
    /// by 100,000 microAlgos. An opt-in transaction is simply an asset transfer with an amount of
    /// 0, both to and from the account opting in.
    pub fn asset_opt_in(
        asset_id: u64,
        fee: MicroAlgos,
        first_valid_round: u64,
        sender: AlgorandAddress,
        genesis_hash: AlgorandHash,
        last_valid_round: Option<u64>,
    ) -> Result<AlgorandTransaction> {
        Ok(Self {
            genesis_hash,
            first_valid_round,
            asset_amount: None,
            sender: sender.clone(),
            asset_receiver: Some(sender),
            transfer_asset_id: Some(asset_id),
            fee: fee.check_if_satisfies_minimum_fee()?.0,
            last_valid_round: Self::calculate_last_valid_round(
                first_valid_round,
                last_valid_round,
            )?,
            txn_type: AlgorandTransactionType::AssetTransfer,
            note: None,
            group: None,
            lease: None,
            amount: None,
            asset_id: None,
            rekey_to: None,
            receiver: None,
            genesis_id: None,
            asset_sender: None,
            asset_parameters: None,
            close_remainder_to: None,
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
        let tx = AlgorandTransaction::asset_opt_in(
            463265200,
            MicroAlgos(1000),
            17_962_294,
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
        let expected_result = "82a3736967c4404e714fa39e98ed14a1220a9fe25efbba3b47ccbf784b28bdff0c5fa1f71d1d289034f1157b97ceb63c0cedb3a6790ba87930f6a7a40d495d41ecde101597cd09a374786e88a461726376c42090826960db089ee5636266600d56a9f41f5d037e5c90a18007e384fc1558603da3666565cd03e8a26676ce01121536a26768c420c061c4d8fc1dbdded2d7604be4568e3f6d041987ac37bde4b620b5ab39248adfa26c76ce0112191ea3736e64c42090826960db089ee5636266600d56a9f41f5d037e5c90a18007e384fc1558603da474797065a56178666572a478616964ce1b9cddb0";
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_sign_asset_transfer_tx() {
        let tx = AlgorandTransaction::asset_transfer(
            463265200,
            MicroAlgos(1000),
            100001337,
            17_962_505,
            AlgorandAddress::from_str("GSKWPLI7YL7OF23F5ET5L7HSFLLJL3F5DUO7AH2HQLOSO4DRRHR76TDQ2I")
                .unwrap(),
            AlgorandHash::mainnet_genesis_hash().unwrap(),
            None,
            AlgorandAddress::from_str("GKT5XX6N45UV3ENMIOAVF7EQQYL77P45XFHYIPBFAJUON7RBUCQPX572TI")
                .unwrap(),
        )
        .unwrap();
        let result = tx
            .sign(&get_sample_algorand_keys())
            .unwrap()
            .to_hex()
            .unwrap();
        let expected_result = "82a3736967c440df49d27ce4b5436e7bee58ffcdbe7ff2fb87b56c96309c997f9b525dcac33a78df3992c0ab4ce92b5da3bd7933b7318fdd15e92043bc6d120047e108e437ab07a374786e89a461616d74ce05f5e639a461726376c42032a7dbdfcde7695d91ac438152fc908617ffbf9db94f843c250268e6fe21a0a0a3666565cd03e8a26676ce01121609a26768c420c061c4d8fc1dbdded2d7604be4568e3f6d041987ac37bde4b620b5ab39248adfa26c76ce011219f1a3736e64c420349567ad1fc2fee2eb65e927d5fcf22ad695ecbd1d1df01f4782dd27707189e3a474797065a56178666572a478616964ce1b9cddb0";
        assert_eq!(result, expected_result);
    }
}
