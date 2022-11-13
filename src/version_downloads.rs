//! <b style="font-variant:small-caps">version_downloads.csv</b>

#![allow(deprecated)] // https://github.com/chronotope/chrono/issues/820#issuecomment-1312651118

use crate::versions::VersionId;
use chrono::{Date, Utc};
use serde_derive::Deserialize;

/// One row of **version_downloads.csv**.
#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    pub version_id: VersionId,
    pub downloads: u64,
    #[serde(deserialize_with = "crate::date::de")]
    pub date: Date<Utc>,
}
