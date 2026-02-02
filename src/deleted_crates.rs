//! <b style="font-variant:small-caps">deleted_crates.csv</b>

use crate::users::UserId;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

/// Primary key of **deleted_crates.csv**.
#[derive(Serialize, Deserialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[serde(transparent)]
#[cfg_attr(not(doc), repr(transparent))]
pub struct DeletedCrateId(pub u32);

/// One row of **deleted_crates.csv**.
#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    /// PRIMARY KEY
    pub id: DeletedCrateId,
    pub name: String,
    #[serde(deserialize_with = "crate::datetime::de")]
    pub created_at: DateTime<Utc>,
    #[serde(deserialize_with = "crate::datetime::de")]
    pub deleted_at: DateTime<Utc>,
    pub deleted_by: Option<UserId>,
    pub message: String,
    #[serde(deserialize_with = "crate::datetime::de")]
    pub available_at: DateTime<Utc>,
}
