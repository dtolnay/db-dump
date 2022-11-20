#![allow(deprecated)] // https://github.com/chronotope/chrono/issues/820#issuecomment-1312651118

use chrono::{Day, NaiveDate, Utc};
use serde::de::{Deserializer, Unexpected, Visitor};
use std::fmt;

struct CratesioDateVisitor;

impl<'de> Visitor<'de> for CratesioDateVisitor {
    type Value = Day<Utc>;

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
            let naive_date = match NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32) {
                Some(naive_date) => naive_date,
                None => break,
            };
            return Ok(Day::new(naive_date, Utc));
        }
        Err(serde::de::Error::invalid_value(
            Unexpected::Str(string),
            &self,
        ))
    }
}

pub(crate) fn de<'de, D>(deserializer: D) -> Result<Day<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(CratesioDateVisitor)
}

#[cfg(test)]
mod tests {
    use chrono::{Day, NaiveDate, Utc};
    use serde::de::value::Error;
    use serde::de::IntoDeserializer;

    #[test]
    fn test_de() {
        let csv = "2020-01-01";
        let deserializer = IntoDeserializer::<Error>::into_deserializer;
        assert_eq!(
            super::de(deserializer(csv)).unwrap(),
            Day::new(NaiveDate::from_ymd(2020, 1, 1), Utc),
        );
    }
}
