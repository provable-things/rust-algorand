use serde::{de::Visitor, Deserialize};

pub struct U8_32Visitor;

impl<'de> Visitor<'de> for U8_32Visitor {
    type Value = [u8; 32];

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a 32 byte array")
    }

    fn visit_bytes<E: serde::de::Error>(self, v: &[u8]) -> std::result::Result<Self::Value, E> {
        if v.len() == 32 {
            let mut bytes = [0; 32];
            bytes.copy_from_slice(v);
            Ok(bytes)
        } else {
            Err(E::custom(format!("Invalid byte array length: {}", v.len())))
        }
    }
}
