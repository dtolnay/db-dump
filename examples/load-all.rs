//! $ cargo run --release --example load-all
//!
//! Load everything. Almost never useful other than for benchmarking.

fn main() -> db_dump::Result<()> {
    let _db = db_dump::load_all("./db-dump.tar.gz")?;

    Ok(())
}
