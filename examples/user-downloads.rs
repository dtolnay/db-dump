//! $ cargo run --release --example user-downloads
//!
//! Computes time series of the fraction of crates.io downloads attributed to a
//! single given user's crates.

use chrono::{Date, Utc};
use std::collections::{BTreeMap as Map, BTreeSet as Set};

const USER: &str = "dtolnay";

#[derive(Default)]
struct Downloads {
    theirs: u64,
    all: u64,
}

fn main() -> db_dump::Result<()> {
    let mut user_id = None;
    let mut crate_owners = Vec::new();
    let mut versions = Vec::new();
    let mut version_downloads = Vec::new();
    db_dump::Loader::new()
        .users(|row| {
            if row.gh_login == USER {
                user_id = Some(row.id);
            }
        })
        .crate_owners(|row| crate_owners.push(row))
        .versions(|row| versions.push(row))
        .version_downloads(|row| version_downloads.push(row))
        .load("./db-dump.tar.gz")?;

    // User id of the crate author we care about.
    let user_id = user_id.expect("no such user");

    // Set of crate ids currently owned by that user.
    let mut their_crates = Set::new();
    for crate_owner in crate_owners {
        if crate_owner.owner_id == user_id {
            their_crates.insert(crate_owner.crate_id);
        }
    }

    // Set of version ids of all of those crates.
    let mut their_versions = Set::new();
    for version in versions {
        if their_crates.contains(&version.crate_id) {
            their_versions.insert(version.id);
        }
    }

    // Add up downloads across that user's crates, as well as total downloads of
    // all crates.
    let mut downloads = Map::<Date<Utc>, Downloads>::new();
    for stat in version_downloads {
        let entry = downloads.entry(stat.date).or_default();
        entry.all += stat.downloads;
        if their_versions.contains(&stat.version_id) {
            entry.theirs += stat.downloads;
        }
    }

    // Print user's downloads as a fraction of total crates.io downloads by day.
    for (date, downloads) in downloads {
        if downloads.theirs > 0 {
            println!(
                "{},{}",
                date,
                downloads.theirs as f64 / downloads.all as f64,
            );
        }
    }

    Ok(())
}
