//! <b style="font-variant:small-caps">versions.csv</b>

use crate::crates::CrateId;
use crate::users::UserId;
use chrono::NaiveDateTime;
use semver::Version;
use serde::de::{Deserializer, Unexpected, Visitor};
use serde_derive::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::BTreeMap as Map;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Primary key of **versions.csv**.
#[derive(Serialize, Deserialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[serde(transparent)]
#[repr(transparent)]
pub struct VersionId(pub u32);

/// One row of **versions.csv**.
#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    /// PRIMARY KEY
    pub id: VersionId,
    pub crate_id: CrateId,
    #[serde(deserialize_with = "version")]
    pub num: Version,
    #[serde(deserialize_with = "crate::datetime::de")]
    pub updated_at: NaiveDateTime,
    #[serde(deserialize_with = "crate::datetime::de")]
    pub created_at: NaiveDateTime,
    pub downloads: u64,
    #[serde(deserialize_with = "features_map")]
    pub features: Map<String, Vec<String>>,
    #[serde(deserialize_with = "crate::bool::de")]
    pub yanked: bool,
    pub license: String,
    pub crate_size: Option<u64>,
    pub published_by: Option<UserId>,
    #[serde(deserialize_with = "checksum", default)]
    pub checksum: Option<[u8; 32]>,
    #[serde(default)]
    pub links: Option<String>,
}

impl Ord for Row {
    fn cmp(&self, other: &Self) -> Ordering {
        VersionId::cmp(&self.id, &other.id)
    }
}

impl PartialOrd for Row {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        VersionId::partial_cmp(&self.id, &other.id)
    }
}

impl Eq for Row {}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        VersionId::eq(&self.id, &other.id)
    }
}

impl Hash for Row {
    fn hash<H: Hasher>(&self, state: &mut H) {
        VersionId::hash(&self.id, state);
    }
}

impl Borrow<VersionId> for Row {
    fn borrow(&self) -> &VersionId {
        &self.id
    }
}

fn compat(string: &str) -> Option<Version> {
    let deprecated = match string {
        "0.0.1-001" => "0.0.1-1",
        "0.3.0-alpha.01" => "0.3.0-alpha.1",
        "0.4.0-alpha.00" => "0.4.0-alpha.0",
        "0.4.0-alpha.01" => "0.4.0-alpha.1",
        _ => return None,
    };
    Some(deprecated.parse().unwrap())
}

struct VersionVisitor;

impl<'de> Visitor<'de> for VersionVisitor {
    type Value = Version;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("semver version")
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
                        "{}: {}",
                        err, string,
                    )))
                }
            }
        }
    }
}

fn version<'de, D>(deserializer: D) -> Result<Version, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(VersionVisitor)
}

struct FeaturesMapVisitor;

impl<'de> Visitor<'de> for FeaturesMapVisitor {
    type Value = Map<String, Vec<String>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("features map")
    }

    fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        serde_json::from_str(string).map_err(serde::de::Error::custom)
    }
}

fn features_map<'de, D>(deserializer: D) -> Result<Map<String, Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(FeaturesMapVisitor)
}

struct ChecksumVisitor;

impl<'de> Visitor<'de> for ChecksumVisitor {
    type Value = Option<[u8; 32]>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("checksum as 64-character hex string")
    }

    fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match string.len() {
            0 => Ok(None),
            64 => {
                let mut checksum = [0u8; 32];
                for i in 0..32 {
                    match u8::from_str_radix(&string[i * 2..][..2], 16) {
                        Ok(byte) => checksum[i] = byte,
                        Err(_) => return Err(E::invalid_value(Unexpected::Str(string), &self)),
                    }
                }
                Ok(Some(checksum))
            }
            _ => Err(E::invalid_value(Unexpected::Str(string), &self)),
        }
    }
}

fn checksum<'de, D>(deserializer: D) -> Result<Option<[u8; 32]>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(ChecksumVisitor)
}
