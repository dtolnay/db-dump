//! <b style="font-variant:small-caps">versions.csv</b>

use crate::crates::CrateId;
use crate::users::UserId;
use chrono::NaiveDateTime;
use semver::Version;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::BTreeMap as Map;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[serde(transparent)]
#[repr(transparent)]
pub struct VersionId(pub u32);

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    /// PRIMARY KEY
    pub id: VersionId,
    pub crate_id: CrateId,
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
