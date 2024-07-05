use serde::de::{Deserializer, Visitor};
use std::fmt;

struct BoolVisitor;

impl<'de> Visitor<'de> for BoolVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("'t' or 'f'")
    }

    fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if string == "t" {
            Ok(true)
        } else if string == "f" {
            Ok(false)
        } else {
            Err(serde::de::Error::unknown_variant(string, &["t", "f"]))
        }
    }
}

pub(crate) fn de<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(BoolVisitor)
}
