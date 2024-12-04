//! <b style="font-variant:small-caps">versions.csv</b>

use crate::crates::CrateId;
use crate::ignore::IgnoredStr;
use crate::users::UserId;
use chrono::{DateTime, Utc};
use semver::{BuildMetadata, Op, Version, VersionReq};
use serde::de::{Deserialize, Deserializer, Unexpected, Visitor};
use serde_derive::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::BTreeMap as Map;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

/// Primary key of **versions.csv**.
#[derive(Serialize, Deserialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[serde(transparent)]
#[cfg_attr(not(doc), repr(transparent))]
pub struct VersionId(pub u32);

/// One row of **versions.csv**.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Row {
    /// PRIMARY KEY
    pub id: VersionId,
    pub crate_id: CrateId,
    pub num: Version,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub downloads: u64,
    pub features: Map<String, Vec<String>>,
    pub yanked: bool,
    pub license: String,
    pub crate_size: Option<u64>,
    pub published_by: Option<UserId>,
    pub checksum: Option<[u8; 32]>,
    pub links: Option<String>,
    pub rust_version: Option<Version>,
    pub has_lib: bool,
    pub bin_names: Vec<String>,
    pub edition: Option<u16>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub repository: Option<String>,
    pub categories: Vec<String>,
    pub keywords: Vec<String>,
}

impl<'de> Deserialize<'de> for Row {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Row {
            id: VersionId,
            crate_id: CrateId,
            #[serde(deserialize_with = "version")]
            num: Version,
            #[serde(default)]
            #[allow(dead_code)]
            num_no_build: IgnoredStr,
            #[serde(deserialize_with = "crate::datetime::de")]
            updated_at: DateTime<Utc>,
            #[serde(deserialize_with = "crate::datetime::de")]
            created_at: DateTime<Utc>,
            downloads: u64,
            #[serde(deserialize_with = "features_map")]
            features: Map<String, Vec<String>>,
            #[serde(deserialize_with = "crate::bool::de")]
            yanked: bool,
            license: String,
            crate_size: Option<u64>,
            published_by: Option<UserId>,
            #[serde(deserialize_with = "checksum", default)]
            checksum: Option<[u8; 32]>,
            #[serde(default)]
            links: Option<String>,
            #[serde(default, deserialize_with = "rust_version")]
            rust_version: Option<Version>,
            #[serde(default, deserialize_with = "has_lib")]
            has_lib: bool,
            #[serde(default, deserialize_with = "bin_names")]
            bin_names: Vec<String>,
            edition: Option<u16>,
            description: Option<String>,
            homepage: Option<String>,
            documentation: Option<String>,
            repository: Option<String>,
            #[serde(default, deserialize_with = "categories")]
            categories: Vec<String>,
            #[serde(default, deserialize_with = "keywords")]
            keywords: Vec<String>,
        }

        let Row {
            id,
            crate_id,
            num,
            num_no_build: _,
            updated_at,
            created_at,
            downloads,
            features,
            yanked,
            license,
            crate_size,
            published_by,
            checksum,
            links,
            rust_version,
            has_lib,
            bin_names,
            edition,
            description,
            homepage,
            documentation,
            repository,
            categories,
            keywords,
        } = Row::deserialize(deserializer)?;
        Ok(Self {
            id,
            crate_id,
            num,
            updated_at,
            created_at,
            downloads,
            features,
            yanked,
            license,
            crate_size,
            published_by,
            checksum,
            links,
            rust_version,
            has_lib,
            bin_names,
            edition,
            description,
            homepage,
            documentation,
            repository,
            categories,
            keywords,
        })
    }
}

impl Ord for Row {
    fn cmp(&self, other: &Self) -> Ordering {
        VersionId::cmp(&self.id, &other.id)
    }
}

impl PartialOrd for Row {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

struct RustVersionVisitor;

impl<'de> Visitor<'de> for RustVersionVisitor {
    type Value = Option<Version>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a compiler version number")
    }

    fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match VersionReq::from_str(string) {
            Ok(mut req) if req.comparators.len() == 1 => {
                let req = req.comparators.pop().unwrap();
                if req.op == Op::Caret {
                    Ok(Some(Version {
                        major: req.major,
                        minor: req.minor.unwrap_or(0),
                        patch: req.patch.unwrap_or(0),
                        pre: req.pre,
                        build: BuildMetadata::EMPTY,
                    }))
                } else {
                    Ok(None)
                }
            }
            Ok(_) => Ok(None),
            Err(parse_error) => Err(E::custom(parse_error)),
        }
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(self)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }
}

fn rust_version<'de, D>(deserializer: D) -> Result<Option<Version>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(RustVersionVisitor)
}

#[derive(Deserialize)]
#[serde(transparent)]
struct HasLib(#[serde(deserialize_with = "crate::bool::de")] bool);

fn has_lib<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match Deserialize::deserialize(deserializer)? {
        Some(HasLib(has_lib)) => Ok(has_lib),
        None => Ok(false),
    }
}

fn bin_names<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    crate::set::optional(deserializer, "binary names set")
}

fn categories<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    crate::set::de(deserializer, "categories set")
}

fn keywords<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    crate::set::de(deserializer, "keywords set")
}
