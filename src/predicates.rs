use std::collections::HashMap;

use crate::algorand_hash::AlgorandHash;

pub fn is_zero_hash(hash: &Option<AlgorandHash>) -> bool {
    match hash {
        Some(hash) => hash.is_zero(),
        _ => false,
    }
}

pub fn is_zero_hash_or_none(hash: &Option<AlgorandHash>) -> bool {
    match hash {
        Some(hash) => hash.is_zero(),
        _ => true,
    }
}

pub fn is_empty_or_none<K, V>(hash_map: &Option<HashMap<K, V>>) -> bool {
    match hash_map {
        Some(hash_map) => hash_map.is_empty(),
        _ => true,
    }
}

pub fn is_false(val: &Option<bool>) -> bool {
    match val {
        Some(val) => !val,
        None => true,
    }
}

pub fn is_zero(num: &u64) -> bool {
    *num == 0
}

pub fn is_zero_option(num: &Option<u64>) -> bool {
    match num {
        Some(val) => is_zero(val),
        None => true,
    }
}

pub fn is_empty_vec<T>(vec: &Option<Vec<T>>) -> bool {
    match vec {
        Some(vec) => vec.is_empty(),
        None => true,
    }
}
