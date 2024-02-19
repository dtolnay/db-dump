//! [![github]](https://github.com/dtolnay/db-dump)&ensp;[![crates-io]](https://crates.io/crates/db-dump)&ensp;[![docs-rs]](https://docs.rs/db-dump)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

#![doc(html_root_url = "https://docs.rs/db-dump/0.5.1")]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::doc_markdown,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_lifetimes,
    clippy::never_loop,
    clippy::too_many_lines,
    clippy::uninlined_format_args,
    clippy::unreadable_literal,
    clippy::unseparated_literal_suffix
)]

extern crate self as db_dump;

mod bool;
mod date;
mod datetime;
mod error;
mod index;
mod load;

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
pub mod version_downloads;
pub mod versions;

pub use crate::error::{Error, Result};
pub use crate::index::Index;
pub use crate::load::{load_all, Loader};

/// A crates.io DB dump with *everything* deserialized into memory. Use
/// [`Loader`] to load only parts of a dump, which is more efficient.
///
/// One of these full dumps can be loaded via [`db_dump::load_all`].
#[derive(Default)]
#[non_exhaustive]
pub struct DbDump {
    /// <table style="width:initial"><tr>
    /// <th>categories.csv</th>
    /// <td>id</td>
    /// <td>category</td>
    /// <td>slug</td>
    /// <td>description</td>
    /// <td>crates_cnt</td>
    /// <td>created_at</td>
    /// <td>path</td>
    /// </tr></table>
    pub categories: Vec<categories::Row>,

    /// <table style="width:initial"><tr>
    /// <th>crate_owners.csv</th>
    /// <td>crate_id</td>
    /// <td>owner_id</td>
    /// <td>created_at</td>
    /// <td>created_by</td>
    /// <td>owner_kind</td>
    /// </tr></table>
    pub crate_owners: Vec<crate_owners::Row>,

    /// <table style="width:initial"><tr>
    /// <th>crates.csv</th>
    /// <td>id</td>
    /// <td>name</td>
    /// <td>updated_at</td>
    /// <td>created_at</td>
    /// <td>downloads</td>
    /// <td>description</td>
    /// <td>homepage</td>
    /// <td>documentation</td>
    /// <td>readme</td>
    /// <td>repository</td>
    /// <td>max_upload_size</td>
    /// <td>max_features</td>
    /// </tr></table>
    pub crates: Vec<crates::Row>,

    /// <table style="width:initial"><tr>
    /// <th>crates_categories.csv</th>
    /// <td>crate_id</td>
    /// <td>category_id</td>
    /// </tr></table>
    pub crates_categories: Vec<crates_categories::Row>,

    /// <table style="width:initial"><tr>
    /// <th>crates_keywords.csv</th>
    /// <td>crate_id</td>
    /// <td>keyword_id</td>
    /// </tr></table>
    pub crates_keywords: Vec<crates_keywords::Row>,

    /// <table style="width:initial"><tr>
    /// <th>dependencies.csv</th>
    /// <td>id</td>
    /// <td>version_id</td>
    /// <td>crate_id</td>
    /// <td>req</td>
    /// <td>optional</td>
    /// <td>default_features</td>
    /// <td>features</td>
    /// <td>target</td>
    /// <td>kind</td>
    /// <td>explicit_name</td>
    /// </tr></table>
    pub dependencies: Vec<dependencies::Row>,

    /// <table style="width:initial"><tr>
    /// <th>keywords.csv</th>
    /// <td>id</td>
    /// <td>keyword</td>
    /// <td>crates_cnt</td>
    /// <td>created_at</td>
    /// </tr></table>
    pub keywords: Vec<keywords::Row>,

    /// <table style="width:initial"><tr>
    /// <th>metadata.csv</th>
    /// <td>total_downloads</td>
    /// </tr></table>
    pub metadata: metadata::Row,

    /// <table style="width:initial"><tr>
    /// <th>reserved_crate_names.csv</th>
    /// <td>name</td>
    /// </tr></table>
    pub reserved_crate_names: Vec<reserved_crate_names::Row>,

    /// <table style="width:initial"><tr>
    /// <th>teams.csv</th>
    /// <td>id</td>
    /// <td>login</td>
    /// <td>github_id</td>
    /// <td>name</td>
    /// <td>avatar</td>
    /// <td>org_id</td>
    /// </tr></table>
    pub teams: Vec<teams::Row>,

    /// <table style="width:initial"><tr>
    /// <th>users.csv</th>
    /// <td>id</td>
    /// <td>gh_login</td>
    /// <td>name</td>
    /// <td>gh_avatar</td>
    /// <td>gh_id</td>
    /// </tr></table>
    pub users: Vec<users::Row>,

    /// <table style="width:initial"><tr>
    /// <th>version_downloads.csv</th>
    /// <td>version_id</td>
    /// <td>downloads</td>
    /// <td>date</td>
    /// </tr></table>
    pub version_downloads: Vec<version_downloads::Row>,

    /// <table style="width:initial"><tr>
    /// <th>versions.csv</th>
    /// <td>id</td>
    /// <td>crate_id</td>
    /// <td>num</td>
    /// <td>updated_at</td>
    /// <td>created_at</td>
    /// <td>downloads</td>
    /// <td>features</td>
    /// <td>yanked</td>
    /// <td>license</td>
    /// <td>crate_size</td>
    /// <td>published_by</td>
    /// <td>checksum</td>
    /// <td>links</td>
    /// <td>rust_version</td>
    /// </tr></table>
    pub versions: Vec<versions::Row>,
}
