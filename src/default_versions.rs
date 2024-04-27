//! <b style="font-variant:small-caps">default_versions.csv</b>

use crate::crates::CrateId;
use crate::versions::VersionId;
use serde_derive::Deserialize;

/// One row of **default_versions.csv**.
#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    pub crate_id: CrateId,
    pub version_id: VersionId,
}
