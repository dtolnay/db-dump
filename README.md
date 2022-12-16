crates.io database dumps
========================

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/db--dump-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/db-dump)
[<img alt="crates.io" src="https://img.shields.io/crates/v/db-dump.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/db-dump)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-db--dump-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/db-dump)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/db-dump/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/dtolnay/db-dump/actions?query=branch%3Amaster)

Library for scripting analyses against crates.io's database dumps.

These database dumps contain all information exposed by the crates.io API
packaged into a single download. An updated dump is published every 24 hours.
The latest dump is available at
*https://<span></span>static.crates.io/db-dump.tar.gz*.

```toml
[dependencies]
db-dump = "0.2"
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
```

<img alt="Crates.io downloads per day (log scale)" src="https://raw.githubusercontent.com/dtolnay/db-dump/master/chart/total-downloads.png">

---

Here is a graph from the **user-downloads** example:

<img alt="Fraction of crates.io downloads that are dtolnay's crates" src="https://raw.githubusercontent.com/dtolnay/db-dump/master/chart/user-downloads.png">

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
