//! $ cargo run --release --example stream-all
//!
//! Load everything without accumulating in memory (unlike load-all). Almost
//! never useful other than for benchmarking.

use std::time::Instant;

fn main() -> db_dump::Result<()> {
    let start = Instant::now();

    db_dump::Loader::new()
        .categories(|_row| {})
        .crate_downloads(|_row| {})
        .crate_owners(|_row| {})
        .crates(|_row| {})
        .crates_categories(|_row| {})
        .crates_keywords(|_row| {})
        .default_versions(|_row| {})
        .deleted_crates(|_row| {})
        .dependencies(|_row| {})
        .keywords(|_row| {})
        .metadata(|_row| {})
        .oauth_github(|_row| {})
        .reserved_crate_names(|_row| {})
        .reserved_usernames(|_row| {})
        .teams(|_row| {})
        .users(|_row| {})
        .version_downloads(|_row| {})
        .versions(|_row| {})
        .load("./db-dump.tar.gz")?;

    let elapsed = start.elapsed();
    println!("{}.{:03}sec", elapsed.as_secs(), elapsed.subsec_millis());
    Ok(())
}
