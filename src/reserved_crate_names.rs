//! <b style="font-variant:small-caps">reserved_crate_names.csv</b>

use serde::Deserialize;

/// One row of **reserved_crate_names.csv**.
#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    /// PRIMARY KEY
    pub name: String,
}
