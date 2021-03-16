//! <b style="font-variant:small-caps">version_authors.csv</b>

use crate::versions::VersionId;
use serde::Deserialize;

/// One row of **version_authors.csv**.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    /// PRIMARY KEY
    pub id: u32,
    pub version_id: VersionId,
    pub name: String,
}
