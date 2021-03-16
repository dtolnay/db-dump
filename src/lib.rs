mod bool;
mod date;
mod datetime;
mod error;
mod load;

pub mod badges;
pub mod categories;
pub mod crate_owners;
pub mod crates;
pub mod crates_categories;
pub mod crates_keywords;
pub mod dependencies;
pub mod keywords;
pub mod metadata;
pub mod reserved_crate_names;
pub mod teams;
pub mod users;
pub mod version_authors;
pub mod version_downloads;
pub mod versions;

pub use crate::error::{Error, Result};
pub use crate::load::{load_all, Loader};

#[derive(Default)]
#[non_exhaustive]
pub struct DbDump {
    pub badges: Vec<badges::Row>,
    pub categories: Vec<categories::Row>,
    pub crate_owners: Vec<crate_owners::Row>,
    pub crates: Vec<crates::Row>,
    pub crates_categories: Vec<crates_categories::Row>,
    pub crates_keywords: Vec<crates_keywords::Row>,
    pub dependencies: Vec<dependencies::Row>,
    pub keywords: Vec<keywords::Row>,
    pub metadata: metadata::Row,
    pub reserved_crate_names: Vec<reserved_crate_names::Row>,
    pub teams: Vec<teams::Row>,
    pub users: Vec<users::Row>,
    pub version_authors: Vec<version_authors::Row>,
    pub version_downloads: Vec<version_downloads::Row>,
    pub versions: Vec<versions::Row>,
}
