//! <b style="font-variant:small-caps">reserved_usernames.csv</b>

use serde_derive::Deserialize;

/// One row of **reserved_usernames.csv**.
#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    /// PRIMARY KEY
    pub username: String,
}
