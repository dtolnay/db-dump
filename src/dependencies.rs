use crate::crates::CrateId;
use crate::versions::VersionId;
use semver::VersionReq;
use serde::de::{Unexpected, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    /// PRIMARY KEY
    pub id: u32,
    pub version_id: VersionId,
    pub crate_id: CrateId,
    pub req: VersionReq,
    #[serde(deserialize_with = "crate::bool::de")]
    pub optional: bool,
    #[serde(deserialize_with = "crate::bool::de")]
    pub default_features: bool,
    #[serde(deserialize_with = "features_set")]
    pub features: Vec<String>,
    pub target: String,
    pub kind: DependencyKind,
}

#[derive(Debug)]
pub enum DependencyKind {
    Normal,
    Build,
    Dev,
}

struct DependencyKindVisitor;

impl<'de> Visitor<'de> for DependencyKindVisitor {
    type Value = DependencyKind;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("dependency kind (0, 1, 2)")
    }

    fn visit_u8<E>(self, kind: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match kind {
            0 => Ok(DependencyKind::Normal),
            1 => Ok(DependencyKind::Build),
            2 => Ok(DependencyKind::Dev),
            _ => Err(serde::de::Error::invalid_value(
                Unexpected::Unsigned(kind as u64),
                &self,
            )),
        }
    }
}

impl<'de> Deserialize<'de> for DependencyKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u8(DependencyKindVisitor)
    }
}

struct FeaturesSetVisitor;

impl<'de> Visitor<'de> for FeaturesSetVisitor {
    type Value = Vec<String>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("features set")
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
        } else {
            Err(serde::de::Error::invalid_value(
                Unexpected::Str(string),
                &self,
            ))
        }
    }
}

fn features_set<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(FeaturesSetVisitor)
}
