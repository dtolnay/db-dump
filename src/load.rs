use crate::error::{err, Result};
use crate::DbDump;
use csv::StringRecord;
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use memmap::Mmap;
use serde::de::DeserializeOwned;
use std::borrow::Cow;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tar::Archive;

/// Perform a streaming load of only relevant database tables.
///
/// # Example
///
/// This example loads just the version_downloads.csv table, in which each row
/// is the download count for a single version of a single crate on a single
/// day. We do not store the rows individually in memory but instead stream from
/// the csv to accumulate just a total count per day across all crates, which
/// requires far less memory.
///
/// ```no_run
/// use chrono::Utc;
/// use db_dump::Date;
/// use std::collections::BTreeMap as Map;
///
/// fn main() -> db_dump::Result<()> {
///     let mut downloads = Map::<Date<Utc>, u64>::new();
///     db_dump::Loader::new()
///         .version_downloads(|row| {
///             *downloads.entry(row.date).or_default() += row.downloads;
///         })
///         .load("./db-dump.tar.gz")?;
///
///     for (date, count) in downloads {
///         println!("{},{}", date, count);
///     }
///
///     Ok(())
/// }
/// ```
#[derive(Default)]
pub struct Loader<'a> {
    categories: Option<Callback<'a, crate::categories::Row>>,
    crate_downloads: Option<Callback<'a, crate::crate_downloads::Row>>,
    crate_owners: Option<Callback<'a, crate::crate_owners::Row>>,
    crates: Option<Callback<'a, crate::crates::Row>>,
    crates_categories: Option<Callback<'a, crate::crates_categories::Row>>,
    crates_keywords: Option<Callback<'a, crate::crates_keywords::Row>>,
    dependencies: Option<Callback<'a, crate::dependencies::Row>>,
    keywords: Option<Callback<'a, crate::keywords::Row>>,
    metadata: Option<Callback<'a, crate::metadata::Row>>,
    reserved_crate_names: Option<Callback<'a, crate::reserved_crate_names::Row>>,
    teams: Option<Callback<'a, crate::teams::Row>>,
    users: Option<Callback<'a, crate::users::Row>>,
    version_downloads: Option<Callback<'a, crate::version_downloads::Row>>,
    versions: Option<Callback<'a, crate::versions::Row>>,
}

struct Callback<'a, T> {
    f: Box<dyn FnMut(T) + 'a>,
    done: bool,
}

impl<'a> Loader<'a> {
    pub fn new() -> Self {
        Loader::default()
    }

    pub fn categories(&mut self, f: impl FnMut(crate::categories::Row) + 'a) -> &mut Self {
        self.categories = Some(Callback::new(f));
        self
    }

    pub fn crate_downloads(
        &mut self,
        f: impl FnMut(crate::crate_downloads::Row) + 'a,
    ) -> &mut Self {
        self.crate_downloads = Some(Callback::new(f));
        self
    }

    pub fn crate_owners(&mut self, f: impl FnMut(crate::crate_owners::Row) + 'a) -> &mut Self {
        self.crate_owners = Some(Callback::new(f));
        self
    }

    pub fn crates(&mut self, f: impl FnMut(crate::crates::Row) + 'a) -> &mut Self {
        self.crates = Some(Callback::new(f));
        self
    }

    pub fn crates_categories(
        &mut self,
        f: impl FnMut(crate::crates_categories::Row) + 'a,
    ) -> &mut Self {
        self.crates_categories = Some(Callback::new(f));
        self
    }

    pub fn crates_keywords(
        &mut self,
        f: impl FnMut(crate::crates_keywords::Row) + 'a,
    ) -> &mut Self {
        self.crates_keywords = Some(Callback::new(f));
        self
    }

    pub fn dependencies(&mut self, f: impl FnMut(crate::dependencies::Row) + 'a) -> &mut Self {
        self.dependencies = Some(Callback::new(f));
        self
    }

    pub fn keywords(&mut self, f: impl FnMut(crate::keywords::Row) + 'a) -> &mut Self {
        self.keywords = Some(Callback::new(f));
        self
    }

    pub fn metadata(&mut self, f: impl FnMut(crate::metadata::Row) + 'a) -> &mut Self {
        self.metadata = Some(Callback::new(f));
        self
    }

    pub fn reserved_crate_names(
        &mut self,
        f: impl FnMut(crate::reserved_crate_names::Row) + 'a,
    ) -> &mut Self {
        self.reserved_crate_names = Some(Callback::new(f));
        self
    }

    pub fn teams(&mut self, f: impl FnMut(crate::teams::Row) + 'a) -> &mut Self {
        self.teams = Some(Callback::new(f));
        self
    }

    pub fn users(&mut self, f: impl FnMut(crate::users::Row) + 'a) -> &mut Self {
        self.users = Some(Callback::new(f));
        self
    }

    pub fn version_downloads(
        &mut self,
        f: impl FnMut(crate::version_downloads::Row) + 'a,
    ) -> &mut Self {
        self.version_downloads = Some(Callback::new(f));
        self
    }

    pub fn versions(&mut self, f: impl FnMut(crate::versions::Row) + 'a) -> &mut Self {
        self.versions = Some(Callback::new(f));
        self
    }

    pub fn load(&mut self, path: impl AsRef<Path>) -> Result<()> {
        do_load(path.as_ref(), self)
    }
}

impl<'a, T> Callback<'a, T> {
    fn new(f: impl FnMut(T) + 'a) -> Self {
        Callback {
            f: Box::new(f),
            done: false,
        }
    }

    fn done(&self) -> bool {
        self.done
    }
}

fn do_load(path: &Path, loader: &mut Loader) -> Result<()> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file) }?;

    let pb = ProgressBar::hidden();
    pb.set_length(mmap.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{wide_bar:.cyan/blue}] {percent}% {msg:>24}")
            .unwrap()
            .progress_chars(". "),
    );
    pb.set_draw_target(ProgressDrawTarget::stderr());
    let input = pb.wrap_read(&*mmap);

    let mut archive = Archive::new(GzDecoder::new(input));
    for entry in archive.entries()? {
        #[deny(unused_variables)]
        let Loader {
            categories,
            crate_downloads,
            crate_owners,
            crates,
            crates_categories,
            crates_keywords,
            dependencies,
            keywords,
            metadata,
            reserved_crate_names,
            teams,
            users,
            version_downloads,
            versions,
        } = loader;

        if categories.as_ref().map_or(true, Callback::done)
            && crate_downloads.as_ref().map_or(true, Callback::done)
            && crate_owners.as_ref().map_or(true, Callback::done)
            && crates.as_ref().map_or(true, Callback::done)
            && crates_categories.as_ref().map_or(true, Callback::done)
            && crates_keywords.as_ref().map_or(true, Callback::done)
            && dependencies.as_ref().map_or(true, Callback::done)
            && keywords.as_ref().map_or(true, Callback::done)
            && metadata.as_ref().map_or(true, Callback::done)
            && reserved_crate_names.as_ref().map_or(true, Callback::done)
            && teams.as_ref().map_or(true, Callback::done)
            && users.as_ref().map_or(true, Callback::done)
            && version_downloads.as_ref().map_or(true, Callback::done)
            && versions.as_ref().map_or(true, Callback::done)
        {
            break;
        }

        let entry = entry?;
        let path = entry.path()?;
        if path.extension().map_or(true, |ext| ext != "csv") {
            continue;
        }

        pb.set_message(match path.file_name() {
            Some(file_name) => Cow::Owned(file_name.to_string_lossy().into_owned()),
            None => Cow::Borrowed(""),
        });

        #[deny(unused_variables)]
        let Loader {
            categories,
            crate_downloads,
            crate_owners,
            crates,
            crates_categories,
            crates_keywords,
            dependencies,
            keywords,
            metadata,
            reserved_crate_names,
            teams,
            users,
            version_downloads,
            versions,
        } = loader;

        let (path, result) = if path.ends_with("badges.csv") {
            continue; // https://github.com/rust-lang/crates.io/pull/8155
        } else if path.ends_with("categories.csv") {
            ("categories", read(categories, entry))
        } else if path.ends_with("crate_downloads.csv") {
            ("crate_downloads", read(crate_downloads, entry))
        } else if path.ends_with("crate_owners.csv") {
            ("crate_owners", read(crate_owners, entry))
        } else if path.ends_with("crates.csv") {
            ("crates", read(crates, entry))
        } else if path.ends_with("crates_categories.csv") {
            ("crates_categories", read(crates_categories, entry))
        } else if path.ends_with("crates_keywords.csv") {
            ("crates_keywords", read(crates_keywords, entry))
        } else if path.ends_with("dependencies.csv") {
            ("dependencies", read(dependencies, entry))
        } else if path.ends_with("keywords.csv") {
            ("keywords", read(keywords, entry))
        } else if path.ends_with("metadata.csv") {
            ("metadata", read(metadata, entry))
        } else if path.ends_with("reserved_crate_names.csv") {
            ("reserved_crate_names", read(reserved_crate_names, entry))
        } else if path.ends_with("teams.csv") {
            ("teams", read(teams, entry))
        } else if path.ends_with("users.csv") {
            ("users", read(users, entry))
        } else if path.ends_with("version_authors.csv") {
            continue; // https://github.com/rust-lang/crates.io/pull/3549
        } else if path.ends_with("version_downloads.csv") {
            ("version_downloads", read(version_downloads, entry))
        } else if path.ends_with("versions.csv") {
            ("versions", read(versions, entry))
        } else {
            if cfg!(db_dump_panic_on_unrecognized_csv) {
                panic!("unimplemented: {}", path.display());
            } else {
                eprintln!("unimplemented: {}", path.display());
            }
            continue;
        };

        if let Err(mut err) = result {
            err.e.path = Some(Path::new(path));
            return Err(err);
        }
    }

    Ok(())
}

pub(crate) trait FromRecord: Sized {
    fn from_record(record: &StringRecord, headers: &StringRecord) -> Result<Self>;
}

impl<T> FromRecord for T
where
    T: DeserializeOwned,
{
    fn from_record(record: &StringRecord, headers: &StringRecord) -> Result<Self> {
        record.deserialize(Some(headers)).map_err(err)
    }
}

fn read<T>(loader: &mut Option<Callback<T>>, entry: impl Read) -> Result<()>
where
    T: FromRecord,
{
    if let Some(loader) = loader {
        let mut csv = csv::Reader::from_reader(entry);
        let headers = csv.headers().map_err(err)?.clone();
        let mut record = StringRecord::new();
        while csv.read_record(&mut record).map_err(err)? {
            let record = T::from_record(&record, &headers)?;
            (loader.f)(record);
        }
        loader.done = true;
    }
    Ok(())
}

/// Deserialize *everything* in a crates.io DB dump into memory.
///
/// This function is equivalent to the following [`Loader`]-based invocation:
///
/// ```
/// # use std::path::Path;
/// # use db_dump::Result;
/// #
/// # struct DbDump {
/// #     categories: Vec<db_dump::categories::Row>,
/// #     crate_owners: Vec<db_dump::crate_owners::Row>,
/// #     versions: Vec<db_dump::versions::Row>,
/// # }
/// #
/// # pub fn load_all(path: impl AsRef<Path>) -> Result<DbDump> {
/// #     let path = path.as_ref();
/// let mut categories = Vec::new();
/// let mut crate_owners = Vec::new();
/// /* ... */
/// let mut versions = Vec::new();
///
/// db_dump::Loader::new()
///     .categories(|row| categories.push(row))
///     .crate_owners(|row| crate_owners.push(row))
///     /* ... */
///     .versions(|row| versions.push(row))
///     .load(path)?;
///
/// Ok(DbDump {
///     categories,
///     crate_owners,
///     /* ... */
///     versions,
/// })
/// # }
/// ```
///
/// Usually whatever you are doing will not require *all* of the information in
/// a dump, in which case utilizing `Loader` to load just what you need can be
/// significantly more efficient.
pub fn load_all(path: impl AsRef<Path>) -> Result<DbDump> {
    do_load_all(path.as_ref())
}

fn do_load_all(path: &Path) -> Result<DbDump> {
    let mut categories = Vec::new();
    let mut crate_downloads = Vec::new();
    let mut crate_owners = Vec::new();
    let mut crates = Vec::new();
    let mut crates_categories = Vec::new();
    let mut crates_keywords = Vec::new();
    let mut dependencies = Vec::new();
    let mut keywords = Vec::new();
    let mut metadata = crate::metadata::Row { total_downloads: 0 };
    let mut reserved_crate_names = Vec::new();
    let mut teams = Vec::new();
    let mut users = Vec::new();
    let mut version_downloads = Vec::new();
    let mut versions = Vec::new();

    let mut loader = Loader {
        categories: Some(Callback::new(|row| categories.push(row))),
        crate_downloads: Some(Callback::new(|row| crate_downloads.push(row))),
        crate_owners: Some(Callback::new(|row| crate_owners.push(row))),
        crates: Some(Callback::new(|row| crates.push(row))),
        crates_categories: Some(Callback::new(|row| crates_categories.push(row))),
        crates_keywords: Some(Callback::new(|row| crates_keywords.push(row))),
        dependencies: Some(Callback::new(|row| dependencies.push(row))),
        keywords: Some(Callback::new(|row| keywords.push(row))),
        metadata: Some(Callback::new(|row| metadata = row)),
        reserved_crate_names: Some(Callback::new(|row| reserved_crate_names.push(row))),
        teams: Some(Callback::new(|row| teams.push(row))),
        users: Some(Callback::new(|row| users.push(row))),
        version_downloads: Some(Callback::new(|row| version_downloads.push(row))),
        versions: Some(Callback::new(|row| versions.push(row))),
    };

    loader.load(path)?;
    drop(loader);

    Ok(DbDump {
        categories,
        crate_downloads,
        crate_owners,
        crates,
        crates_categories,
        crates_keywords,
        dependencies,
        keywords,
        metadata,
        reserved_crate_names,
        teams,
        users,
        version_downloads,
        versions,
    })
}
