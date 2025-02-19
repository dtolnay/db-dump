use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use serde::de::{Deserializer, Unexpected, Visitor};
use std::fmt;

// The timestamps in the db dump CSV do not mention a time zone, but in reality
// they refer to UTC.
struct CratesioDateTimeVisitor;

impl<'de> Visitor<'de> for CratesioDateTimeVisitor {
    type Value = DateTime<Utc>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("datetime in format 'YYYY-MM-DD HH:MM:SS.SSSSSS'")
    }

    fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // NaiveDateTime::parse_from_str(string, "%Y-%m-%d %H:%M:%S%.6f")
        'err: {
            let string = string.strip_suffix("+00").unwrap_or(string);
            if string.len() < 19 {
                break 'err;
            }
            let year: u16 = match string[0..4].parse() {
                Ok(year) => year,
                Err(_) => break 'err,
            };
            if string[4..5] != *"-" {
                break 'err;
            }
            let month: u8 = match string[5..7].parse() {
                Ok(month) => month,
                Err(_) => break 'err,
            };
            if string[7..8] != *"-" {
                break 'err;
            }
            let day: u8 = match string[8..10].parse() {
                Ok(day) => day,
                Err(_) => break 'err,
            };
            let Some(naive_date) = NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32)
            else {
                break 'err;
            };
            if string[10..11] != *" " {
                break 'err;
            }
            let hour: u8 = match string[11..13].parse() {
                Ok(hour) => hour,
                Err(_) => break 'err,
            };
            if string[13..14] != *":" {
                break 'err;
            }
            let min: u8 = match string[14..16].parse() {
                Ok(min) => min,
                Err(_) => break 'err,
            };
            if string[16..17] != *":" {
                break 'err;
            }
            let sec: u8 = match string[17..19].parse() {
                Ok(sec) => sec,
                Err(_) => break 'err,
            };
            let micro: u32 = if string.len() == 19 {
                0
            } else if string[19..20] != *"." || string.len() > 26 {
                break 'err;
            } else if let Ok(micro) = string[20..].parse::<u32>() {
                let trailing_zeros = 26 - string.len() as u32;
                micro * 10u32.pow(trailing_zeros)
            } else {
                break 'err;
            };
            let Some(naive_time) =
                NaiveTime::from_hms_micro_opt(hour as u32, min as u32, sec as u32, micro)
            else {
                break 'err;
            };
            let naive_date_time = NaiveDateTime::new(naive_date, naive_time);
            return Ok(Utc.from_utc_datetime(&naive_date_time));
        }
        Err(serde::de::Error::invalid_value(
            Unexpected::Str(string),
            &self,
        ))
    }
}

pub(crate) fn de<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(CratesioDateTimeVisitor)
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
    use serde::de::value::Error;
    use serde::de::IntoDeserializer;

    #[test]
    fn test_de() {
        let csv = "2020-01-01 12:11:10.999999";
        let deserializer = IntoDeserializer::<Error>::into_deserializer;
        assert_eq!(
            super::de(deserializer(csv)).unwrap(),
            Utc.from_utc_datetime(&NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                NaiveTime::from_hms_micro_opt(12, 11, 10, 999999).unwrap(),
            )),
        );

        let csv = "2020-01-01 12:11:10.99";
        assert_eq!(
            super::de(deserializer(csv)).unwrap(),
            Utc.from_utc_datetime(&NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                NaiveTime::from_hms_micro_opt(12, 11, 10, 990000).unwrap(),
            )),
        );

        let csv = "2020-01-01 12:11:10";
        assert_eq!(
            super::de(deserializer(csv)).unwrap(),
            Utc.from_utc_datetime(&NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                NaiveTime::from_hms_micro_opt(12, 11, 10, 0).unwrap(),
            )),
        );
    }
}
