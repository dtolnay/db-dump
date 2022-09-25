//! <b style="font-variant:small-caps">crates_keywords.csv</b>

use crate::crates::CrateId;
use crate::keywords::KeywordId;
use serde::Deserialize;

/// One row of **crates_keywords.csv**.
#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    pub crate_id: CrateId,
    pub keyword_id: KeywordId,
}
