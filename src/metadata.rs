use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    pub total_downloads: u64,
}
