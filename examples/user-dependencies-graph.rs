//! $ cargo run --release --example user-dependencies2
//!
//! Computes time series of the fraction of deps on crates.io which point to a
//! given user's crates.

use std::collections::{BTreeMap as Map, BTreeSet as Set};

const USER: &str = "dtolnay";

fn main() -> db_dump::Result<()> {
    let mut user_id = None;
    let mut crate_owners = Vec::new();
    let mut dependencies = Map::new();
    let mut versions = Vec::new();
    db_dump::Loader::new()
        .users(|row| {
            if row.gh_login == USER {
                user_id = Some(row.id);
            }
        })
        .crate_owners(|row| crate_owners.push(row))
        .dependencies(|row| {
            dependencies
                .entry(row.version_id)
                .or_insert_with(Vec::new)
                .push(row);
        })
        .versions(|row| {
            if !row.yanked {
                versions.push(row);
            }
        })
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

    let mut total_deps = 0usize;
    let mut their_deps = 0usize;
    let mut last_printed_ratio = 0.0..=0.0;
    let mut latest_version = Map::new();

    versions.sort_by_key(|v| v.created_at);

    for version in versions {
        let no_deps = Vec::new();
        if let Some(prev) = latest_version.insert(version.crate_id, version.id) {
            for dep in dependencies.get(&prev).unwrap_or(&no_deps) {
                total_deps -= 1;
                their_deps -= their_crates.contains(&dep.crate_id) as usize;
            }
        }
        for dep in dependencies.get(&version.id).unwrap_or(&no_deps) {
            total_deps += 1;
            their_deps += their_crates.contains(&dep.crate_id) as usize;
        }
        if total_deps != 0 {
            let ratio = their_deps as f64 / total_deps as f64;
            if !last_printed_ratio.contains(&ratio) {
                println!("{},{:.3}", version.created_at.naive_utc(), ratio * 100.0);
                last_printed_ratio = ratio * 0.99999..=ratio * 1.00001;
            }
        }
    }

    eprintln!(
        "{} / {} ({:.02}%)",
        their_deps,
        total_deps,
        (their_deps as f64 / total_deps as f64) * 100.0,
    );
    Ok(())
}
