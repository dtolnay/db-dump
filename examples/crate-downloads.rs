//! $ cargo run --release --example crate-downloads
//!
//! Computes time series of downloads of one specific crate.

use chrono::{Date, Utc};
use std::collections::{BTreeMap as Map, BTreeSet as Set};

const CRATE: &str = "syn";

fn main() -> db_dump::Result<()> {
    let mut crate_id = None;
    let mut versions = Vec::new();
    let mut version_downloads = Vec::new();
    db_dump::Loader::new()
        .crates(|row| {
            if row.name == CRATE {
                crate_id = Some(row.id);
            }
        })
        .versions(|row| versions.push(row))
        .version_downloads(|row| version_downloads.push(row))
        .load("./db-dump.tar.gz")?;

    // Crate id of the crate we care about.
    let crate_id = crate_id.expect("no such crate");

    // Set of all version ids corresponding to that crate.
    let mut version_ids = Set::new();
    for version in versions {
        if version.crate_id == crate_id {
            version_ids.insert(version.id);
        }
    }

    // Add up downloads across all version of the crate by day.
    let mut downloads = Map::<Date<Utc>, u64>::new();
    for stat in version_downloads {
        if version_ids.contains(&stat.version_id) {
            *downloads.entry(stat.date).or_default() += stat.downloads;
        }
    }

    for (date, count) in downloads {
        println!("{},{}", date, count);
    }

    Ok(())
}
