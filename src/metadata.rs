//! <b style="font-variant:small-caps">metadata.csv</b>

use serde::Deserialize;

/// One row of **metadata.csv**.
#[derive(Deserialize, Clone, Default, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    pub total_downloads: u64,
}
