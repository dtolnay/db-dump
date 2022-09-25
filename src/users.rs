//! <b style="font-variant:small-caps">users.csv</b>

use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// Primary key of **users.csv**.
#[derive(Serialize, Deserialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[serde(transparent)]
#[repr(transparent)]
pub struct UserId(pub u32);

/// One row of **users.csv**.
#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    /// PRIMARY KEY
    pub id: UserId,
    pub gh_login: String,
    pub name: Option<String>,
    pub gh_avatar: String,
    pub gh_id: i32,
}

impl Ord for Row {
    fn cmp(&self, other: &Self) -> Ordering {
        UserId::cmp(&self.id, &other.id)
    }
}

impl PartialOrd for Row {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        UserId::partial_cmp(&self.id, &other.id)
    }
}

impl Eq for Row {}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        UserId::eq(&self.id, &other.id)
    }
}

impl Hash for Row {
    fn hash<H: Hasher>(&self, state: &mut H) {
        UserId::hash(&self.id, state);
    }
}

impl Borrow<UserId> for Row {
    fn borrow(&self) -> &UserId {
        &self.id
    }
}
