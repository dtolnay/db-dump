crates.io database dumps
========================

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/db--dump-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/db-dump)
[<img alt="crates.io" src="https://img.shields.io/crates/v/db-dump.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/db-dump)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-db--dump-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/db-dump)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/dtolnay/db-dump/CI/master?style=for-the-badge" height="20">](https://github.com/dtolnay/db-dump/actions?query=branch%3Amaster)

Library for scripting analyses against crates.io's database dumps.

These database dumps contain all information exposed by the crates.io API
packaged into a single download. An updated dump is published every 24 hours.
The latest dump is available at
*https://<span></span>static.crates.io/db-dump.tar.gz*.

```toml
[dependencies]
db-dump = "0.1"
```

<br>

## Examples

The *examples/* directory of this repo contains several runnable example
analyses.

<table>
<tr><th><a href="examples/total-downloads.rs">total&#8209;downloads</a></th><td>
Computes time series of total downloads by day across all crates on
crates.io</td></tr>
<tr><th><a href="examples/crate-downloads.rs">crate&#8209;downloads</a></th><td>
Computes time series of downloads of one specific crate</td></tr>
<tr><th><a href="examples/top-crates.rs">top&#8209;crates</a></th><td>
Computes the top few most directly depended upon crates</td></tr>
<tr><th><a href="examples/user-dependencies.rs">user&#8209;dependencies</a></th><td>
Computes the percentage of crates.io which depends directly on at least one
crate by the specified user</td></tr>
<tr><th><a href="examples/user-downloads.rs">user&#8209;downloads</a></th><td>
Computes time series of the fraction of crates.io downloads attributed to a
single given user's crates</td></tr>
</table>

Each of these examples can be run using Cargo once you've downloaded a recent
database dump:

```console
$ wget https://static.crates.io/db-dump.tar.gz
$ cargo run --release --example total-downloads
```

---

Here is the implementation of the most basic example, **total-downloads**, and
graph of the resulting table. It shows crates.io download rate doubling every 9
months, or equivalently 10&times; every 2.5 years!

```rust
use chrono::NaiveDate;
use std::collections::BTreeMap as Map;

fn main() -> db_dump::Result<()> {
    let mut downloads = Map::<NaiveDate, u64>::new();
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
```

<img alt="Crates.io downloads per day (log scale)" src="https://user-images.githubusercontent.com/1940490/111275478-6ac87000-85f3-11eb-85b7-c35ed5e1257a.png">

---

Here is a graph from the **user-downloads** example:

<img alt="Fraction of crates.io downloads that are dtolnay's crates" src="https://user-images.githubusercontent.com/1940490/111275874-df9baa00-85f3-11eb-9c73-110f9943b0c9.png">

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
