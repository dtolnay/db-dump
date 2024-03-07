// Usage:
//
//     cargo run --release -- path/to/dir
//
//     cargo run --release -- path/to/db-dump-????-??-??-??????.tar.gz
//
// The program takes one or more arguments, which may be directories or
// individual tgz files. For directories, the program will assume the naming
// convention above for the contents.
//
// Pro tip: when passing files, your shell's glob expansion is your friend,
// using either `*` or `?` as shown above.
//
// The output file will be named db-dump-????-??-??-??????-concat.tar.gz using
// the date and time that is the largest among the input files, or can be
// specified explicitly using the --out flag.

#![allow(
    clippy::cast_lossless,
    clippy::cast_sign_loss,
    clippy::let_underscore_untyped,
    clippy::never_loop,
    clippy::too_many_lines,
    clippy::trivially_copy_pass_by_ref,
    clippy::uninlined_format_args
)]

use anyhow::{bail, format_err, Result};
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use clap::Parser;
use csv::StringRecord;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use memmap::Mmap;
use serde::de::{Deserializer, Unexpected, Visitor};
use serde::ser::Serializer;
use serde_derive::{Deserialize, Serialize};
use std::cmp;
use std::collections::BTreeMap as Map;
use std::fmt;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Component, Path, PathBuf};
use std::str;
use tar::Archive;

#[derive(Parser, Debug)]
#[command(name = "db-dump-concat", author, version)]
struct Opt {
    #[arg(short, long)]
    out: Option<PathBuf>,

    #[arg(required = true)]
    dumps: Vec<PathBuf>,
}

const GLOB: &str = "db-dump-????-??-??-??????.tar.gz";

#[derive(Default)]
struct Entry {
    datestamp: String,
    content: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct VersionDownloads {
    #[serde(serialize_with = "ser_naive_date", deserialize_with = "de_naive_date")]
    pub date: NaiveDate,
    pub version_id: u32,
    pub downloads: u64,
}

fn main() -> Result<()> {
    let opt = Opt::parse();

    // Expand directories into individual files matching glob.
    let mut paths = Vec::new();
    let glob = glob::Pattern::new(GLOB).unwrap();
    for path in opt.dumps {
        if path.is_dir() {
            let dir_start = paths.len();
            for entry in path.read_dir()? {
                let entry = entry?;
                if glob.matches_path(Path::new(&entry.file_name())) {
                    paths.push(entry.path());
                }
            }
            if dir_start == paths.len() {
                bail!("no files matching {}", path.join(GLOB).display());
            }
            paths[dir_start..].sort();
        } else {
            paths.push(path);
        }
    }

    let mut max_datestamp = String::new();
    let mut version_downloads: Map<(NaiveDate, u32), u64> = Map::new();
    let mut other_files: Map<PathBuf, Entry> = Map::new();

    let stderr = io::stderr();
    let mut stderr = stderr.lock();

    for path in &paths {
        let file = File::open(path).map_err(|e| format_err!("{}: {}", path.display(), e))?;
        let mmap = unsafe { Mmap::map(&file) }?;
        let mut archive = Archive::new(GzDecoder::new(&*mmap));
        for file in archive.entries()? {
            let mut file = file?;
            let path = file.path()?;
            if path.extension().map_or(true, |ext| ext != "csv") {
                continue;
            }
            let _ = writeln!(stderr, "loading {}", path.display());
            let mut components = path.components();
            let datestamp = match components.next() {
                Some(Component::Normal(datestamp)) => datestamp.to_string_lossy(),
                _ => continue,
            };
            if *datestamp > *max_datestamp {
                max_datestamp = datestamp.as_ref().to_owned();
            }
            let relative: PathBuf = components.collect();
            if relative.ends_with("version_downloads.csv") {
                let mut csv = csv::Reader::from_reader(file);
                let headers = csv.headers()?.clone();
                let mut record = StringRecord::new();
                while csv.read_record(&mut record)? {
                    let record: VersionDownloads = record.deserialize(Some(&headers))?;
                    let downloads = version_downloads
                        .entry((record.date, record.version_id))
                        .or_default();
                    *downloads = cmp::max(*downloads, record.downloads);
                }
            } else {
                let entry = other_files.entry(relative).or_default();
                if *datestamp > *entry.datestamp {
                    entry.datestamp = datestamp.into_owned();
                    entry.content.clear();
                    file.read_to_end(&mut entry.content)?;
                }
            }
        }
    }

    let datestamp_fmt = "%Y-%m-%d-%H%M%S";
    let timestamp = NaiveDateTime::parse_from_str(&max_datestamp, datestamp_fmt)?
        .and_utc()
        .timestamp();

    let _ = writeln!(stderr, "serializing csv");
    let mut csv_writer = csv::Writer::from_writer(Vec::new());
    for ((date, version_id), downloads) in version_downloads {
        csv_writer.serialize(VersionDownloads {
            date,
            version_id,
            downloads,
        })?;
    }
    other_files.insert(
        PathBuf::from("data/version_downloads.csv"),
        Entry {
            datestamp: max_datestamp.clone(),
            content: csv_writer.into_inner()?,
        },
    );

    let out_path = if let Some(out) = opt.out {
        out
    } else {
        PathBuf::from(format!("db-dump-{}-concat.tar.gz", max_datestamp))
    };
    let out_file = File::create(out_path)?;
    let gz = GzEncoder::new(out_file, Compression::best());
    let mut tar = tar::Builder::new(gz);

    for (relative, entry) in other_files {
        let _ = writeln!(stderr, "writing {}", relative.display());
        let path = Path::new(&max_datestamp).join(relative);
        let mut header = tar::Header::new_gnu();
        header.set_mode(0o600);
        header.set_mtime(timestamp as u64);
        header.set_size(entry.content.len() as u64);
        tar.append_data(&mut header, path, entry.content.as_slice())?;
    }

    tar.finish()?;
    Ok(())
}

fn ser_naive_date<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut bytes = *b"0000-00-00";
    write!(&mut bytes[0..4], "{:04}", date.year()).unwrap();
    write!(&mut bytes[5..7], "{:02}", date.month()).unwrap();
    write!(&mut bytes[8..10], "{:02}", date.day()).unwrap();
    serializer.serialize_str(str::from_utf8(&bytes).unwrap())
}

fn de_naive_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    struct NaiveDateVisitor;

    impl<'de> Visitor<'de> for NaiveDateVisitor {
        type Value = NaiveDate;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("date in format 'YYYY-MM-DD'")
        }

        fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            loop {
                if string.len() != 10 {
                    break;
                }
                let year: u16 = match string[0..4].parse() {
                    Ok(year) => year,
                    Err(_) => break,
                };
                if string[4..5] != *"-" {
                    break;
                }
                let month: u8 = match string[5..7].parse() {
                    Ok(month) => month,
                    Err(_) => break,
                };
                if string[7..8] != *"-" {
                    break;
                }
                let day: u8 = match string[8..10].parse() {
                    Ok(day) => day,
                    Err(_) => break,
                };
                match NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32) {
                    Some(naive_date) => return Ok(naive_date),
                    None => break,
                }
            }
            Err(serde::de::Error::invalid_value(
                Unexpected::Str(string),
                &self,
            ))
        }
    }

    deserializer.deserialize_str(NaiveDateVisitor)
}
