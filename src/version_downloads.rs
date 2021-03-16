use crate::versions::VersionId;
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    pub version_id: VersionId,
    pub downloads: u64,
    #[serde(deserialize_with = "crate::date::de")]
    pub date: NaiveDate,
}
