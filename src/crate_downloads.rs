//! <b style="font-variant:small-caps">crate_downloads.csv</b>

use crate::crates::CrateId;
use serde_derive::Deserialize;

/// One row of **crate_downloads.csv**.
#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    pub crate_id: CrateId,
    pub downloads: u64,
}
