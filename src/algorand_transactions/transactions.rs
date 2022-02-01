use derive_more::{Constructor, Deref};
use serde::{Deserialize, Serialize};

use crate::{
    algorand_address::AlgorandAddress,
    algorand_transactions::{
        transaction::AlgorandTransaction,
        transaction_json::AlgorandTransactionJson,
    },
    algorand_types::Result,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Deref, Constructor)]
pub struct AlgorandTransactions(Vec<AlgorandTransaction>);

impl AlgorandTransactions {
    pub fn from_jsons(jsons: &[AlgorandTransactionJson]) -> Result<Self> {
        Ok(Self(
            jsons
                .iter()
                .map(|tx_json| AlgorandTransaction::from_json(&tx_json))
                .collect::<Result<Vec<AlgorandTransaction>>>()?,
        ))
    }

    pub fn get_asset_to_txs(&self, address: &AlgorandAddress) -> Self {
        let needle = Some(address.clone());
        Self(
            self.iter()
                .filter(|tx| tx.asset_receiver == needle)
                .cloned()
                .collect::<Vec<AlgorandTransaction>>(),
        )
    }
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Deref, Constructor)]
pub struct AlgorandTransactionsJson(Vec<AlgorandTransactionJson>);

#[cfg(test)]
impl AlgorandTransactions {
    pub fn to_json(&self) -> Result<AlgorandTransactionsJson> {
        Ok(AlgorandTransactionsJson(
            self.0
                .iter()
                .map(|x| x.to_json())
                .collect::<Result<Vec<AlgorandTransactionJson>>>()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use serde_json::json;

    use super::*;
    use crate::algorand_transactions::{
        test_utils::get_sample_txs_n,
        transaction_type::AlgorandTransactionType,
    };

    #[test]
    fn should_get_asset_to_txs() {
        let receiver =
            AlgorandAddress::from_str("SDHQI6LW76DVY46KHDQDQG4MM4H4CCPRM3QI3FHXHD2XAMBMQMG54VF2Y4")
                .unwrap();
        let txs = get_sample_txs_n(0);
        let x = AlgorandTransactions::new(
            txs.clone()
                .iter()
                .filter(|tx| tx.txn_type == Some(AlgorandTransactionType::AssetTransfer))
                .cloned()
                .collect::<Vec<AlgorandTransaction>>(),
        );
        let results = txs.get_asset_to_txs(&receiver);
        let numResults = results.len();
        let expectedNumResult = 1;
        assert_eq!(numResults, expectedNumResult);
        let tx_str = "{\"asset-transfer-transaction\":{\"amount\":0,\"asset-id\":27165954,\"close-amount\":0,\"receiver\":\"SDHQI6LW76DVY46KHDQDQG4MM4H4CCPRM3QI3FHXHD2XAMBMQMG54VF2Y4\"},\"close-rewards\":0,\"closing-amount\":0,\"confirmed-round\":17962555,\"fee\":1000,\"first-valid\":17962553,\"genesis-hash\":\"wGHE2Pwdvd7S12BL5FaOP20EGYesN73ktiC1qzkkit8=\",\"genesis-id\":\"mainnet-v1.0\",\"id\":\"JU4SWMUQ3Q2JNS6GDY2JDQFB3CXWS3GFBRB2W3XYGOJSMODXYRWQ\",\"intra-round-offset\":6,\"last-valid\":17963553,\"note\":\"gaZzdHJpbmfaAdwwNStwRHN2ZUNlQ0tlVHhpVkFZRjgzZlU5REU5NDN6UkN5azZMcmtWWkVkaE5IVmhNbWxNWW5JeFlVZDJVbTFRVFVZcmEzazRZVEpJTnpWWk4xZEVja0poTVZKUFZWTkVkMmxrY0VJNVFuSkdURlJuVUhwMk5FVnBaSGRHV2l0Q1l6aDBVemg1UjFOTVRIRjVRbVJPSzJVcmVuQTVaVkozTkZkaWQzTmtWRlZSYldoT1VucDNkek5tZDNnNVpWSkVSa3RJSzNWME5Xa3lRWFZzU1hSMmVrWjRWek5IV0NzeVVtWndORlF3YVZSNlUwSnNMMDgyTTNGRlVVeHlUVzlZUkdSc05rTkRSVE5KYTFKTFlpdEpjekF5VTJscE1tOHJjV3AzZGt4cU5ucHZPVGxsVlVWQ05UaHJNVUpoT1dGVFEwdEdXa1p0TW5sMGEyOUJSa3B5WjBKTmJ6UjJWSFZ2WmxsM2NrVjRZVnBtYlRkUFFtNVpNVXNyVm1NeFkyMUdiR0pTY2tSQ1VrczJVMU13VWtGbVNteEhaVmc1VDFsSmRqQnlTazUzVjBkalRsQklWM0U1SzI5RFpVaHpOVEIzVlRkYVkyRXhTM0JUTTJrdlRYSjZOSEZ5WXowPQ==\",\"receiver-rewards\":0,\"round-time\":1639242410,\"sender\":\"ZW3ISEHZUHPO7OZGMKLKIIMKVICOUDRCERI454I3DB2BH52HGLSO67W754\",\"sender-rewards\":0,\"signature\":{\"sig\":\"DmjBiax9+V4UStI06IVDzBnEY+0oORkYaqs0ELXrqh6ZdCBcsqaBqW8QZAFP/JqrU7wEW+JPbLPd+Sw4dtqRBg==\"},\"tx-type\":\"axfer\"}";
        let expected_result = AlgorandTransaction::from_str(tx_str).unwrap();
        let result = results[0].clone();
        assert_eq!(result, expected_result);
    }
}
