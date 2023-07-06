//! $ cargo run --release --example user-dependencies
//!
//! Computes the percentage of crates.io which depends directly on at least one
//! crate by the specified user.

use std::collections::btree_map::Entry;
use std::collections::{BTreeMap as Map, BTreeSet as Set};

const USER: &str = "dtolnay";

fn main() -> db_dump::Result<()> {
    // Map of crate id to the most recently published version of that crate.
    let mut most_recent = Map::new();

    let mut user_id = None;
    let mut crates = 0;
    let mut crate_owners = Vec::new();
    let mut dependencies = Vec::new();
    db_dump::Loader::new()
        .users(|row| {
            if row.gh_login == USER {
                user_id = Some(row.id);
            }
        })
        .crates(|_row| crates += 1)
        .crate_owners(|row| crate_owners.push(row))
        .dependencies(|row| dependencies.push(row))
        .versions(|row| match most_recent.entry(row.crate_id) {
            Entry::Vacant(entry) => {
                entry.insert(row);
            }
            Entry::Occupied(mut entry) => {
                if row.created_at > entry.get().created_at {
                    entry.insert(row);
                }
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

    // Set of version ids which are the most recently published of their crate.
    let most_recent = Set::from_iter(most_recent.values().map(|version| version.id));

    // Set of version ids which depend directly on at least one crate by the
    // user.
    let mut dep_on_them = Set::new();
    for dep in dependencies {
        if their_crates.contains(&dep.crate_id) {
            dep_on_them.insert(dep.version_id);
        }
    }

    // Number of crates whose most recent version depends on at least one crate
    // by the user.
    let result = dep_on_them.intersection(&most_recent).count();

    println!(
        "{} / {} = {:.1}%",
        result,
        crates,
        100.0 * result as f64 / crates as f64,
    );

    Ok(())
}
