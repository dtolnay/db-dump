//! <b style="font-variant:small-caps">oauth_github.csv</b>

use crate::users::UserId;
use serde_derive::Deserialize;

/// One row of **oauth_github.csv**.
#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    pub user_id: UserId,
    pub account_id: i32,
    pub login: String,
    pub avatar: String,
}
