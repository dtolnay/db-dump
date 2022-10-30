//! $ cargo run --release --example load-all
//!
//! Load everything. Almost never useful other than for benchmarking.

use std::time::Instant;

fn main() -> db_dump::Result<()> {
    let start = Instant::now();

    let db = db_dump::load_all("./db-dump.tar.gz")?;

    let elapsed = start.elapsed();
    println!("{}.{:03}sec", elapsed.as_secs(), elapsed.subsec_millis());
    drop(db);
    Ok(())
}
