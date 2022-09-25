//! <b style="font-variant:small-caps">version_downloads.csv</b>

use crate::versions::VersionId;
use chrono::NaiveDate;
use serde_derive::Deserialize;

/// One row of **version_downloads.csv**.
#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    pub version_id: VersionId,
    pub downloads: u64,
    #[serde(deserialize_with = "crate::date::de")]
    pub date: NaiveDate,
}
