//! <b style="font-variant:small-caps">teams.csv</b>

use serde_derive::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// Primary key of **teams.csv**.
#[derive(Serialize, Deserialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[serde(transparent)]
#[repr(transparent)]
pub struct TeamId(pub u32);

/// One row of **teams.csv**.
#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    /// PRIMARY KEY
    pub id: TeamId,
    /// UNIQUE
    pub login: String,
    /// UNIQUE
    pub github_id: u32,
    pub name: String,
    pub avatar: String,
    pub org_id: Option<u32>,
}

impl Ord for Row {
    fn cmp(&self, other: &Self) -> Ordering {
        TeamId::cmp(&self.id, &other.id)
    }
}

impl PartialOrd for Row {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        TeamId::partial_cmp(&self.id, &other.id)
    }
}

impl Eq for Row {}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        TeamId::eq(&self.id, &other.id)
    }
}

impl Hash for Row {
    fn hash<H: Hasher>(&self, state: &mut H) {
        TeamId::hash(&self.id, state);
    }
}

impl Borrow<TeamId> for Row {
    fn borrow(&self) -> &TeamId {
        &self.id
    }
}
