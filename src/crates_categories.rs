//! <b style="font-variant:small-caps">crates_categories.csv</b>

use crate::categories::CategoryId;
use crate::crates::CrateId;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    pub crate_id: CrateId,
    pub category_id: CategoryId,
}
