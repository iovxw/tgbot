use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;

/// Telegram Integer type
pub type Integer = i64;

/// Telegram Float type
pub type Float = f32;

/// Represents a unique message identifier
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct MessageId {
    /// Unique message identifier
    pub message_id: Integer,
}

macro_rules! impl_bool_type {
    ($name:ident = $value:ident) => {
        #[derive(Clone, Copy)]
        pub(crate) struct $name;

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_bool($value)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let b = bool::deserialize(deserializer)?;
                if b == $value {
                    Ok(Self)
                } else {
                    Err(D::Error::invalid_value(Unexpected::Bool(b), &stringify!($value)))
                }
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                fmt::Debug::fmt(&$value, formatter)
            }
        }
    };
}

impl_bool_type!(True = true);
impl_bool_type!(False = false);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_bool_ser {
        ($fn:ident($name:ident = $value:ident)) => {
            #[test]
            fn $fn() {
                let value = serde_json::to_string(&$name).unwrap();
                assert_eq!(value, stringify!($value));
            }
        };
    }

    macro_rules! test_bool_de {
        ($( #[should_panic = $msg:literal] )? $fn:ident($name:ident = $value:ident)) => {
            #[test]
            $( #[should_panic = $msg] )?
            fn $fn() {
                serde_json::from_str::<$name>(stringify!($value)).unwrap();
            }
        };
    }

    test_bool_ser!(serialize_true(True = true));
    test_bool_ser!(serialize_false(False = false));

    test_bool_de!(deserialize_true(True = true));
    test_bool_de! {
        #[should_panic = "invalid value: boolean `false`, expected true"]
        deserialize_true_unexpected(True = false)
    }
    test_bool_de!(deserialize_false(False = false));
    test_bool_de! {
        #[should_panic = "invalid value: boolean `true`, expected false"]
        deserialize_false_unexpected(False = true)
    }
}
