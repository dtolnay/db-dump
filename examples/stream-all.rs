//! $ cargo run --release --example stream-all
//!
//! Load everything without accumulating in memory (unlike load-all). Almost
//! never useful other than for benchmarking.

use std::time::Instant;

fn main() -> db_dump::Result<()> {
    let start = Instant::now();

    db_dump::Loader::new()
        .categories(drop)
        .crate_downloads(drop)
        .crate_owners(drop)
        .crates(drop)
        .crates_categories(drop)
        .crates_keywords(drop)
        .default_versions(drop)
        .deleted_crates(drop)
        .dependencies(drop)
        .keywords(drop)
        .metadata(drop)
        .oauth_github(drop)
        .reserved_crate_names(drop)
        .reserved_usernames(drop)
        .teams(drop)
        .users(drop)
        .version_downloads(drop)
        .versions(drop)
        .load("./db-dump.tar.gz")?;

    let elapsed = start.elapsed();
    println!("{}.{:03}sec", elapsed.as_secs(), elapsed.subsec_millis());
    Ok(())
}
