use paste::paste;
use serde::de::Visitor;

macro_rules! make_byte_array_visitors {
    ($($num:expr),*) => {
        paste! {
            $(
                pub struct [<U8_ $num Visitor>];

                impl<'de> Visitor<'de> for [<U8_ $num Visitor>] {
                    type Value = [u8; $num];

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        let s = format!("a {} byte array", $num);
                        formatter.write_str(&s)
                    }

                    fn visit_bytes<E: serde::de::Error>(self, v: &[u8]) -> std::result::Result<Self::Value, E> {
                        if v.len() == $num {
                            let mut bytes = [0; $num];
                            bytes.copy_from_slice(v);
                            Ok(bytes)
                        } else {
                            Err(E::custom(format!("Invalid byte array length: {}", v.len())))
                        }
                    }
                }
            )*
        }
    }
}

make_byte_array_visitors!(32, 64);
