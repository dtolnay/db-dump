//! $ cargo run --release --example find-timestamp -- 2023-01-13T23:42:25.800721Z
//! Reverse lookup what crate got published at a particular time. Useful for
//! attributing discontinuities in a `cargo tally` graph.

use chrono::DateTime;
use db_dump::crates::CrateId;
use semver::Version;
use std::collections::BTreeMap as Map;
use std::env;
use std::io::{self, Write as _};
use std::process;

fn main() -> db_dump::Result<()> {
    let mut query = Vec::new();
    for arg in env::args_os().skip(1) {
        let arg = arg.to_str().unwrap();
        let datetime = DateTime::parse_from_rfc3339(arg).unwrap();
        query.push(datetime.to_utc());
    }

    if query.is_empty() {
        let _ = writeln!(
            io::stderr(),
            "Usage: cargo run --release --example find-timestamp -- 2023-01-13T23:42:25.800721Z",
        );
        process::exit(1);
    }

    let mut crates: Map<CrateId, String> = Map::new();
    let mut versions: Vec<(CrateId, Version)> = Vec::new();
    db_dump::Loader::new()
        .crates(|row| {
            crates.insert(row.id, row.name);
        })
        .versions(|row| {
            if query.contains(&row.created_at) {
                versions.push((row.crate_id, row.num));
            }
        })
        .load("./db-dump.tar.gz")?;

    for (crate_id, version) in versions {
        println!("{} v{}", crates[&crate_id], version);
    }

    Ok(())
}
