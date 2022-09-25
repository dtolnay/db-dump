//! <b style="font-variant:small-caps">crates.csv</b>

use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// Primary key of **crates.csv**.
#[derive(Serialize, Deserialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[serde(transparent)]
#[repr(transparent)]
pub struct CrateId(pub u32);

/// One row of **crates.csv**.
#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    /// PRIMARY KEY
    pub id: CrateId,
    pub name: String,
    #[serde(deserialize_with = "crate::datetime::de")]
    pub updated_at: NaiveDateTime,
    #[serde(deserialize_with = "crate::datetime::de")]
    pub created_at: NaiveDateTime,
    pub downloads: u64,
    pub description: String,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub readme: Option<String>,
    pub repository: Option<String>,
    pub max_upload_size: Option<u64>,
}

impl Ord for Row {
    fn cmp(&self, other: &Self) -> Ordering {
        CrateId::cmp(&self.id, &other.id)
    }
}

impl PartialOrd for Row {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        CrateId::partial_cmp(&self.id, &other.id)
    }
}

impl Eq for Row {}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        CrateId::eq(&self.id, &other.id)
    }
}

impl Hash for Row {
    fn hash<H: Hasher>(&self, state: &mut H) {
        CrateId::hash(&self.id, state);
    }
}

impl Borrow<CrateId> for Row {
    fn borrow(&self) -> &CrateId {
        &self.id
    }
}
