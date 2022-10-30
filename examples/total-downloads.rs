//! $ cargo run --release --example total-downloads
//!
//! Computes time series of total downloads by day across all crates on
//! crates.io.

use chrono::{Date, Utc};
use std::collections::BTreeMap as Map;

fn main() -> db_dump::Result<()> {
    let mut downloads = Map::<Date<Utc>, u64>::new();
    db_dump::Loader::new()
        .version_downloads(|row| {
            *downloads.entry(row.date).or_default() += row.downloads;
        })
        .load("./db-dump.tar.gz")?;

    for (date, count) in downloads {
        println!("{},{}", date, count);
    }

    Ok(())
}
