//! <b style="font-variant:small-caps">dependencies.csv</b>

use crate::crates::CrateId;
use crate::versions::VersionId;
use semver::VersionReq;
use serde::de::{Deserialize, Deserializer, Unexpected, Visitor};
use serde_derive::Deserialize;
use std::fmt;

/// One row of **dependencies.csv**.
#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    /// PRIMARY KEY
    pub id: u32,
    pub version_id: VersionId,
    pub crate_id: CrateId,
    #[serde(deserialize_with = "version_req")]
    pub req: VersionReq,
    #[serde(deserialize_with = "crate::bool::de")]
    pub optional: bool,
    #[serde(deserialize_with = "crate::bool::de")]
    pub default_features: bool,
    #[serde(deserialize_with = "features_set")]
    pub features: Vec<String>,
    pub target: String,
    pub kind: DependencyKind,
    #[serde(default)]
    pub explicit_name: Option<String>,
}

#[derive(Copy, Clone, Debug)]
pub enum DependencyKind {
    /// kind=0
    Normal,
    /// kind=1
    Build,
    /// kind=2
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

fn compat(string: &str) -> Option<VersionReq> {
    let deprecated = match string {
        "^0-.11.0" => "^0.11.0",
        "^0.1-alpha.0" => "^0.1.0-alpha.0",
        "^0.51-oldsyn" => "^0.51.1-oldsyn",
        "~2.0-2.2" => ">=2.0, <=2.2",
        _ => return None,
    };
    Some(deprecated.parse().unwrap())
}

struct VersionReqVisitor;

impl<'de> Visitor<'de> for VersionReqVisitor {
    type Value = VersionReq;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("semver version req")
    }

    fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match string.parse() {
            Ok(version) => Ok(version),
            Err(err) => {
                if let Some(version) = compat(string) {
                    Ok(version)
                } else {
                    Err(serde::de::Error::custom(format_args!(
                        "{}: req {}",
                        err, string,
                    )))
                }
            }
        }
    }
}

fn version_req<'de, D>(deserializer: D) -> Result<VersionReq, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(VersionReqVisitor)
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
