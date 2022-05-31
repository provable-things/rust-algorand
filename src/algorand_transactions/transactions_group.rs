use serde::{Deserialize, Serialize};
use sha2::Digest;

use crate::{
    algorand_errors::AlgorandError,
    algorand_types::{Byte, Bytes, Result},
    AlgorandHash,
    AlgorandKeys,
    AlgorandSignedTransaction,
    AlgorandTransaction,
};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TxGroup {
    #[serde(rename = "txlist", default)]
    tx_group_hashes: Vec<AlgorandHash>,

    #[serde(skip_serializing)]
    txs: Vec<AlgorandTransaction>,

    #[serde(skip_serializing)]
    group_id: AlgorandHash,
}

impl TxGroup {
    const MAX_TX_GROUP_SIZE: usize = 16;

    pub fn new(transactions: &[AlgorandTransaction]) -> Result<TxGroup> {
        transactions
            .iter()
            .map(|tx| tx.to_raw_tx_id())
            .collect::<Result<Vec<AlgorandHash>>>()
            .and_then(|raw_tx_ids| {
                let tx_group = Self {
                    group_id: AlgorandHash::default(),
                    txs: vec![],
                    tx_group_hashes: raw_tx_ids.clone(),
                };

                let group_id = tx_group.compute_group_id()?;

                let txs_with_group_ids_assigned = transactions
                    .iter()
                    .map(|tx| tx.assign_group_id(group_id))
                    .collect::<Vec<AlgorandTransaction>>();

                Ok(Self {
                    group_id,
                    tx_group_hashes: raw_tx_ids,
                    txs: txs_with_group_ids_assigned,
                })
            })
    }

    pub fn compute_group_id(&self) -> Result<AlgorandHash> {
        if self.tx_group_hashes.is_empty() {
            return Err(AlgorandError::Custom("Empty transactions list".to_string()));
        }
        if self.tx_group_hashes.len() > Self::MAX_TX_GROUP_SIZE {
            return Err(AlgorandError::Custom(
                "Too  many transactions in group!".to_string(),
            ));
        }
        let hashed = sha2::Sha512_256::digest(self.encode_for_hashing()?);
        AlgorandHash::from_slice(hashed.as_slice())
    }

    fn to_msg_pack_bytes(&self) -> Result<Bytes> {
        Ok(rmp_serde::to_vec_named(&self)?)
    }

    fn prefix_group_byte(bytes: &[Byte]) -> Bytes {
        let suffix = bytes;
        let mut prefix = b"TG".to_vec();
        prefix.extend_from_slice(suffix);
        prefix
    }

    pub fn encode_for_hashing(&self) -> Result<Bytes> {
        self.to_msg_pack_bytes()
            .map(|ref msg_pack_bytes| Self::prefix_group_byte(msg_pack_bytes))
    }

    pub fn to_hex(txns: Vec<AlgorandSignedTransaction>) -> Result<String> {
        Ok(txns
            .iter()
            .map(|t| t.to_hex())
            .collect::<Result<Vec<String>>>()?
            .join(""))
    }

    pub fn sign_transactions(&self, keys: &[AlgorandKeys]) -> Result<String> {
        let num_keys = keys.len();
        match num_keys {
            1 => {
                // NOTE: We assume that all txs are to be signed with this one key...
                self.txs
                    .iter()
                    .map(|tx| tx.sign(&keys[0]))
                    .collect::<Result<Vec<AlgorandSignedTransaction>>>()
            },
            num_keys if num_keys == self.txs.len() => {
                // NOTE: We assume that there's a key for each tx, so we use those...
                self.txs
                    .iter()
                    .zip(keys.iter())
                    .map(|(tx, key)| tx.sign(key))
                    .collect::<Result<Vec<AlgorandSignedTransaction>>>()
            },
            _ => Err(format!(
                "Please provide either ONE private key, or {} private keys! (IE: one for each tx!)",
                num_keys
            )
            .into()),
        }
        .and_then(Self::to_hex)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::{
        algorand_applications::algorand_application_args::AlgorandApplicationArg,
        algorand_hash::AlgorandHash,
        test_utils::{get_sample_algorand_address, get_sample_algorand_keys},
        AlgorandAddress,
        MicroAlgos,
    };

    #[test]
    fn should_assign_group_id() {
        let tx1 = AlgorandTransaction::new_payment_tx(
            1000000,
            MicroAlgos(230000),
            None,
            17962505,
            AlgorandAddress::from_str("IL5YIRUX577LJ5FVOMATHTB6XR7KQDJTGWG24VLDGFHU2NOWKH67UL2ULI")
                .unwrap(),
            AlgorandAddress::from_str("3XOLRWTASJY25KA6PVMAC3MQBWY4RW3HRAKTSL6ZXJJEJA4B2ODQP3OWGA")
                .unwrap(),
            AlgorandHash::testnet_genesis_hash().unwrap(),
            Some(17963505),
        )
        .unwrap();
        let tx2 = AlgorandTransaction::asset_transfer(
            123456789,
            MicroAlgos(244000),
            1000000,
            None,
            17_962_505,
            AlgorandAddress::from_str("3XOLRWTASJY25KA6PVMAC3MQBWY4RW3HRAKTSL6ZXJJEJA4B2ODQP3OWGA")
                .unwrap(),
            AlgorandHash::testnet_genesis_hash().unwrap(),
            Some(17963505),
            AlgorandAddress::from_str("IL5YIRUX577LJ5FVOMATHTB6XR7KQDJTGWG24VLDGFHU2NOWKH67UL2ULI")
                .unwrap(),
        )
        .unwrap();
        assert_eq!(
            tx1.to_id().unwrap(),
            "IDJRB47H4PIUOMLWOSRQMXSX4GLP7OWIG7DVE67NN6KWBDGQOH4Q"
        );
        assert_eq!(
            tx2.to_id().unwrap(),
            "IWCO45FDR2IUU6ZUFORAD5ZVQ3R6JK5VWQWD4VJFJ6EZK5OCPKZQ"
        );
        let txs_group = TxGroup::new(&[tx1, tx2]).unwrap();
        let expected_group = "d9420d6df510ca93139938e6fda2fa91a3738840a8e88775c4d6b7e73c4072b4";
        assert_eq!(hex::encode(txs_group.group_id.to_bytes()), expected_group);
        txs_group
            .txs
            .iter()
            .for_each(|tx| assert_eq!(hex::encode(tx.group().unwrap().to_bytes()), expected_group));
    }

    #[test]
    fn should_concat_signed_transactions() {
        let tx1 = AlgorandTransaction::asset_transfer(
            19999,
            MicroAlgos(1000),
            1000000,
            None,
            21_682_035,
            AlgorandAddress::from_str("SCBGSYG3BCPOKY3CMZQA2VVJ6QPV2A36LSIKDAAH4OCPYFKYMA65KIOP7U")
                .unwrap(),
            AlgorandHash::testnet_genesis_hash().unwrap(),
            None,
            AlgorandAddress::from_str("IL5YIRUX577LJ5FVOMATHTB6XR7KQDJTGWG24VLDGFHU2NOWKH67UL2ULI")
                .unwrap(),
        )
        .unwrap();
        assert_eq!(
            tx1.to_id().unwrap(),
            "FN7OG2DDLXJCPCBZGHFRVCR6CS62AFQ47RIRDWP7PC2H4XJXAFNA"
        );
        let mut args: Vec<AlgorandApplicationArg> = Vec::new();
        let arg1: &str = "2022-05-20T12:53:59.000Z";
        let arg2: i64 = 1234567890;
        args.push(AlgorandApplicationArg(arg1.as_bytes().to_vec()));
        args.push(AlgorandApplicationArg(arg2.to_be_bytes().to_vec()));
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
        let tx2 = AlgorandTransaction::application_call_noop(
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
        assert_eq!(
            tx2.to_id().unwrap(),
            "3S5V64XDP6SF6OKCNSIMHUSEWJVAJYHBG5HJGTEMYF3ZJ2L76ZPQ"
        );
        let group = TxGroup::new(&[tx1.clone(), tx2.clone()]).unwrap();
        let expected_group = "e37a82859898dc3df4525d7379a313702ce418341ce601d67f4ef266c63e9141";
        assert_eq!(hex::encode(group.group_id.to_bytes()), expected_group);
        group
            .txs
            .iter()
            .for_each(|tx| assert_eq!(hex::encode(tx.group().unwrap().to_bytes()), expected_group));
        let result = group
            .sign_transactions(&[get_sample_algorand_keys()])
            .unwrap();
        let expected_result = "82a3736967c440aa8e3e06eddc79054d01120663a7c7e79485b56577896c31aa6284d4062b23ddd0b749f39edb353b0cadb81afd9651bfa533b4d4c988e7c97309d38878a92000a374786e8aa461616d74ce000f4240a461726376c42042fb844697effeb4f4b5730133cc3ebc7ea80d33358dae5563314f4d35d651fda3666565cd03e8a26676ce014ad773a26768c4204863b518a4b3c84ec810f22d4f1081cb0f71f059a7ac20dec62f7f70e5093a22a3677270c420e37a82859898dc3df4525d7379a313702ce418341ce601d67f4ef266c63e9141a26c76ce014adb5ba3736e64c42090826960db089ee5636266600d56a9f41f5d037e5c90a18007e384fc1558603da474797065a56178666572a478616964cd4e1f82a3736967c4400d21eaceb069838fcb773bfef7f96a8cd7a38561a008f4353db9ebb22f3c1f6ac0199fe2b521f70dffb10d91bff9be21763c520090e6ba3d772deac11d1f3301a374786e8ba46170616192c418323032322d30352d32305431323a35333a35392e3030305ac40800000000499602d2a46170617492c42032a7dbdfcde7695d91ac438152fc908617ffbf9db94f843c250268e6fe21a0a0c420c3a3d439ff6c61367a15c4d2872ffebac9a6c7a03c273812bc64de26726b76d5a46170666191ce075bcd15a461706964ce0565c844a3666565cd03e8a26676ce014ad773a26768c4204863b518a4b3c84ec810f22d4f1081cb0f71f059a7ac20dec62f7f70e5093a22a3677270c420e37a82859898dc3df4525d7379a313702ce418341ce601d67f4ef266c63e9141a26c76ce014adb5ba3736e64c42090826960db089ee5636266600d56a9f41f5d037e5c90a18007e384fc1558603da474797065a46170706c";
        assert_eq!(result, expected_result);
    }
}
