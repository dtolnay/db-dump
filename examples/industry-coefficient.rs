//! $ cargo run --release --example industry-coefficient
//!
//! Computes a coefficient for each crate that is:
//!   • HIGH if the crate is disproportionately often downloaded on weekDAYS,
//!   • and LOW if the crate is disproportionately often downloaded on weekENDS.

use chrono::{Datelike, Duration, Weekday};
use db_dump::crates::CrateId;
use db_dump::versions::VersionId;
use std::collections::BTreeMap as Map;

const DOWNLOADS_CUTOFF: u64 = 1_000_000;

#[derive(Default)]
struct Downloads {
    weekday: u64,
    weekend: u64,
}

fn main() -> db_dump::Result<()> {
    let mut crates: Map<CrateId, String> = Map::new();
    let mut versions: Map<VersionId, CrateId> = Map::new();
    let mut version_downloads = Vec::new();
    db_dump::Loader::new()
        .crates(|row| {
            crates.insert(row.id, row.name);
        })
        .versions(|row| {
            versions.insert(row.id, row.crate_id);
        })
        .version_downloads(|row| version_downloads.push(row))
        .load("./db-dump.tar.gz")?;

    let max_date = version_downloads.iter().map(|row| row.date).max().unwrap();
    let start_date = max_date - Duration::weeks(6);

    // Add up downloads by crate by date
    let mut downloads: Map<CrateId, Downloads> = Map::new();
    for row in version_downloads {
        // Deliberately cut out the largest date in the db-dump, because the
        // data is partial.
        if row.date >= start_date && row.date < max_date {
            let crate_id = versions[&row.version_id];
            let downloads = downloads.entry(crate_id).or_insert_with(Downloads::default);
            match row.date.weekday() {
                Weekday::Tue | Weekday::Wed | Weekday::Thu => downloads.weekday += row.downloads,
                Weekday::Sat | Weekday::Sun => downloads.weekend += row.downloads,
                // Disregard these to reduce some boundary effect from
                // downloaders not being perfectly aligned with UTC.
                Weekday::Mon | Weekday::Fri => {}
            }
        }
    }

    let mut downloads_vec = Vec::new();
    let mut total = Downloads::default();
    for (crate_id, downloads) in downloads {
        total.weekday += downloads.weekday;
        total.weekend += downloads.weekend;
        let crate_name = &crates[&crate_id];
        if downloads.weekend > 0
            && (downloads.weekday + downloads.weekend >= DOWNLOADS_CUTOFF || crate_name == "cxx")
        {
            let coefficient = downloads.weekday as f64 / downloads.weekend as f64;
            downloads_vec.push((crate_name, coefficient));
        }
    }

    let mean = total.weekday as f64 / total.weekend as f64;
    downloads_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for (crate_name, coefficient) in downloads_vec {
        println!("{:>36}  {:+.4}", crate_name, coefficient - mean);
    }

    Ok(())
}
