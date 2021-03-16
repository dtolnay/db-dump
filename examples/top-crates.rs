//! cargo run --release --example top-crates
//!
//! Computes the top few most directly depended upon crates.

use db_dump::crates::CrateId;
use db_dump::versions::VersionId;
use std::cmp::Reverse;
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap as Map, BTreeSet as Set};
use std::iter::FromIterator;

const N: usize = 12;

fn main() -> db_dump::Result<()> {
    // Map of crate id to the most recently published version of that crate.
    let mut most_recent = Map::new();

    let mut crates = Set::new();
    let mut crate_owners = Vec::new();
    let mut dependencies = Vec::new();
    db_dump::Loader::new()
        .crates(|row| {
            crates.insert(row);
        })
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

    // Set of version ids which are the most recently published of their crate.
    let most_recent = Set::from_iter(most_recent.values().map(|version| version.id));

    // Set of (version id, dependency crate id) pairs to avoid double-counting
    // cases where a crate has both a normal dependency and dev-dependency or
    // build-dependency on the same dependency crate.
    let mut unique_dependency_edges = Set::<(VersionId, CrateId)>::new();

    // Map of crate id to how many other crates' most recent version depends on
    // that crate.
    let mut count = Map::<CrateId, usize>::new();
    for dep in dependencies {
        if most_recent.contains(&dep.version_id)
            && unique_dependency_edges.insert((dep.version_id, dep.crate_id))
        {
            *count.entry(dep.crate_id).or_default() += 1;
        }
    }

    // Quickselect and sort the top N crates by reverse dependency count.
    let mut sort = Vec::from_iter(count);
    let sort_by_count = |&(_crate, count): &_| Reverse(count);
    sort.select_nth_unstable_by_key(N - 1, sort_by_count);
    sort[..N].sort_unstable_by_key(sort_by_count);

    for (id, count) in sort.iter().take(N) {
        let crate_name = &crates.get(id).unwrap().name;
        println!("{},{}", crate_name, count);
    }

    Ok(())
}
