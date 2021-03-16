use crate::categories::CategoryId;
use crate::crates::CrateId;
use crate::keywords::KeywordId;
use crate::teams::TeamId;
use crate::users::UserId;
use crate::versions::VersionId;
use crate::DbDump;
use fnv::FnvHashMap as Map;
use once_cell::sync::OnceCell;

impl DbDump {
    pub fn index<'a>(&'a self) -> Index<'a> {
        Index::new(self)
    }
}

pub struct Index<'a> {
    db: &'a DbDump,
    categories: OnceCell<Map<CategoryId, u32>>,
    crates: OnceCell<Map<CrateId, u32>>,
    keywords: OnceCell<Map<KeywordId, u32>>,
    teams: OnceCell<Map<TeamId, u32>>,
    users: OnceCell<Map<UserId, u32>>,
    versions: OnceCell<Map<VersionId, u32>>,
}

impl<'a> Index<'a> {
    pub fn new(db: &'a DbDump) -> Self {
        Index {
            db,
            categories: OnceCell::new(),
            crates: OnceCell::new(),
            keywords: OnceCell::new(),
            teams: OnceCell::new(),
            users: OnceCell::new(),
            versions: OnceCell::new(),
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
