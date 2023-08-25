use crate::categories::CategoryId;
use crate::crates::CrateId;
use crate::keywords::KeywordId;
use crate::teams::TeamId;
use crate::users::UserId;
use crate::versions::VersionId;
use crate::DbDump;
use fnv::FnvHashMap as Map;
use std::sync::OnceLock;

impl DbDump {
    /// Create a lazy index of those tables that have a unique ID column as
    /// primary key.
    ///
    /// This call does no work up front. Each index is built lazily upon first
    /// access through one of the `Index` struct's methods.
    pub fn index<'a>(&'a self) -> Index<'a> {
        Index::new(self)
    }
}

/// Lazy index of those tables that have a unique ID column as primary key.
///
/// # Example
///
/// This example prints the top 5 most downloaded crates in each of the top 20
/// most popular categories.
///
/// ```no_run
/// use db_dump::categories::CategoryId;
/// use db_dump::crates::CrateId;
/// use db_dump::DbDump;
/// use std::cmp::Reverse;
/// use std::collections::BTreeMap as Map;
///
/// fn main() -> db_dump::Result<()> {
///     let mut db = DbDump::default();
///     let ref mut crates = db.crates;
///     let mut categories = Vec::new();
///     let mut crates_by_category = Map::<CategoryId, Vec<CrateId>>::new();
///     db_dump::Loader::new()
///         .crates(|row| crates.push(row))
///         .categories(|row| categories.push(row))
///         .crates_categories(|row| {
///             crates_by_category
///                 .entry(row.category_id)
///                 .or_default()
///                 .push(row.crate_id)
///         })
///         .load("./db-dump.tar.gz")?;
///
///     // Lazy index to perform lookups by crate id.
///     let index = db.index();
///
///     // Sort categories descending by number of crates, to print most popular
///     // categories first.
///     categories.sort_unstable_by_key(|category| Reverse(category.crates_cnt));
///
///     for category in categories.iter().take(20) {
///         // Get the list of crates in this category.
///         let mut crates = crates_by_category.remove(&category.id).unwrap_or_default();
///
///         // Sort crates list by download count descending.
///         crates.sort_unstable_by_key(|&id| Reverse(index.krate(id).downloads));
///
///         // Print top 5 most downloaded crates in category.
///         print!("{}", category.slug);
///         for crate_id in crates.into_iter().take(5) {
///             print!(",{}", index.krate(crate_id).name);
///         }
///         println!();
///     }
///
///     Ok(())
/// }
/// ```
pub struct Index<'a> {
    db: &'a DbDump,
    categories: OnceLock<Map<CategoryId, u32>>,
    crates: OnceLock<Map<CrateId, u32>>,
    keywords: OnceLock<Map<KeywordId, u32>>,
    teams: OnceLock<Map<TeamId, u32>>,
    users: OnceLock<Map<UserId, u32>>,
    versions: OnceLock<Map<VersionId, u32>>,
}

impl<'a> Index<'a> {
    /// This call does no work up front. Each index is built lazily upon first
    /// access through one of the methods below.
    pub fn new(db: &'a DbDump) -> Self {
        Index {
            db,
            categories: OnceLock::new(),
            crates: OnceLock::new(),
            keywords: OnceLock::new(),
            teams: OnceLock::new(),
            users: OnceLock::new(),
            versions: OnceLock::new(),
        }
    }

    pub fn category(&self, id: CategoryId) -> &'a crate::categories::Row {
        &self.db.categories[*self
            .categories
            .get_or_init(|| {
                self.db
                    .categories
                    .iter()
                    .enumerate()
                    .map(|(i, row)| (row.id, i as u32))
                    .collect()
            })
            .get(&id)
            .unwrap_or_else(|| panic!("no such category id={}", id.0))
            as usize]
    }

    pub fn krate(&self, id: CrateId) -> &'a crate::crates::Row {
        &self.db.crates[*self
            .crates
            .get_or_init(|| {
                self.db
                    .crates
                    .iter()
                    .enumerate()
                    .map(|(i, row)| (row.id, i as u32))
                    .collect()
            })
            .get(&id)
            .unwrap_or_else(|| panic!("no such crate id={}", id.0))
            as usize]
    }

    pub fn keyword(&self, id: KeywordId) -> &'a crate::keywords::Row {
        &self.db.keywords[*self
            .keywords
            .get_or_init(|| {
                self.db
                    .keywords
                    .iter()
                    .enumerate()
                    .map(|(i, row)| (row.id, i as u32))
                    .collect()
            })
            .get(&id)
            .unwrap_or_else(|| panic!("no such keyword id={}", id.0))
            as usize]
    }

    pub fn team(&self, id: TeamId) -> &'a crate::teams::Row {
        &self.db.teams[*self
            .teams
            .get_or_init(|| {
                self.db
                    .teams
                    .iter()
                    .enumerate()
                    .map(|(i, row)| (row.id, i as u32))
                    .collect()
            })
            .get(&id)
            .unwrap_or_else(|| panic!("no such team id={}", id.0)) as usize]
    }

    pub fn user(&self, id: UserId) -> &'a crate::users::Row {
        &self.db.users[*self
            .users
            .get_or_init(|| {
                self.db
                    .users
                    .iter()
                    .enumerate()
                    .map(|(i, row)| (row.id, i as u32))
                    .collect()
            })
            .get(&id)
            .unwrap_or_else(|| panic!("no such user id={}", id.0)) as usize]
    }

    pub fn version(&self, id: VersionId) -> &'a crate::versions::Row {
        &self.db.versions[*self
            .versions
            .get_or_init(|| {
                self.db
                    .versions
                    .iter()
                    .enumerate()
                    .map(|(i, row)| (row.id, i as u32))
                    .collect()
            })
            .get(&id)
            .unwrap_or_else(|| panic!("no such version id={}", id.0))
            as usize]
    }
}

impl CategoryId {
    pub fn lookup<'a>(self, index: &Index<'a>) -> &'a crate::categories::Row {
        index.category(self)
    }
}

impl CrateId {
    pub fn lookup<'a>(self, index: &Index<'a>) -> &'a crate::crates::Row {
        index.krate(self)
    }
}

impl KeywordId {
    pub fn lookup<'a>(self, index: &Index<'a>) -> &'a crate::keywords::Row {
        index.keyword(self)
    }
}

impl TeamId {
    pub fn lookup<'a>(self, index: &Index<'a>) -> &'a crate::teams::Row {
        index.team(self)
    }
}

impl UserId {
    pub fn lookup<'a>(self, index: &Index<'a>) -> &'a crate::users::Row {
        index.user(self)
    }
}

impl VersionId {
    pub fn lookup<'a>(self, index: &Index<'a>) -> &'a crate::versions::Row {
        index.version(self)
    }
}
