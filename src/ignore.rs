use serde::de::{Deserialize, Deserializer, Visitor};
use std::fmt;

#[derive(Default)]
pub(crate) struct IgnoredStr;

impl<'de> Deserialize<'de> for IgnoredStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(IgnoredStr)
    }
}

impl<'de> Visitor<'de> for IgnoredStr {
    type Value = IgnoredStr;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string")
    }

    fn visit_str<E>(self, _ignored: &str) -> Result<Self::Value, E> {
        Ok(IgnoredStr)
    }
}
