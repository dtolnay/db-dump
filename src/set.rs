use serde::de::{Deserializer, Unexpected, Visitor};
use std::fmt;

struct SetVisitor<'a> {
    expecting: &'a str,
    optional: bool,
}

impl<'de, 'a> Visitor<'de> for SetVisitor<'a> {
    type Value = Vec<String>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.expecting)
    }

    fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if string.starts_with('{') && string.ends_with('}') {
            let csv = &string[1..string.len() - 1];
            if csv.is_empty() {
                Ok(Vec::new())
            } else {
                Ok(csv.split(',').map(str::to_owned).collect())
            }
        } else if self.optional && string.is_empty() {
            Ok(Vec::new())
        } else {
            Err(serde::de::Error::invalid_value(
                Unexpected::Str(string),
                &self,
            ))
        }
    }
}

pub(crate) fn de<'de, D>(deserializer: D, expecting: &str) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(SetVisitor {
        expecting,
        optional: false,
    })
}

pub(crate) fn optional<'de, D>(deserializer: D, expecting: &str) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(SetVisitor {
        expecting,
        optional: true,
    })
}
