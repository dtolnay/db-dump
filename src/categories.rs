//! <b style="font-variant:small-caps">categories.csv</b>

use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// Primary key of **categories.csv**.
#[derive(Serialize, Deserialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[serde(transparent)]
#[cfg_attr(not(doc), repr(transparent))]
pub struct CategoryId(pub u32);

/// One row of **categories.csv**.
#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    /// PRIMARY KEY
    pub id: CategoryId,
    /// UNIQUE
    pub category: String,
    /// UNIQUE
    pub slug: String,
    pub description: String,
    pub crates_cnt: u32,
    #[serde(deserialize_with = "crate::datetime::de")]
    pub created_at: DateTime<Utc>,
    pub path: String,
}

impl Ord for Row {
    fn cmp(&self, other: &Self) -> Ordering {
        CategoryId::cmp(&self.id, &other.id)
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
        CategoryId::eq(&self.id, &other.id)
    }
}

impl Hash for Row {
    fn hash<H: Hasher>(&self, state: &mut H) {
        CategoryId::hash(&self.id, state);
    }
}

impl Borrow<CategoryId> for Row {
    fn borrow(&self) -> &CategoryId {
        &self.id
    }
}
