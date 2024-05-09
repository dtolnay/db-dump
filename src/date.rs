use chrono::{Datelike, IsoWeek, NaiveDate, TimeDelta, TimeZone, Utc, Weekday};
use serde::de::{Deserialize, Deserializer, Unexpected, Visitor};
use std::cmp::Ordering;
use std::fmt::{self, Debug, Display};
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// Represents the range of local timestamps between midnight and the adjacent
/// midnight in the given timezone.
#[derive(Copy, Clone)]
pub struct Date<Tz> {
    naive: NaiveDate,
    tz: Tz,
}

impl Date<Utc> {
    /// Makes a new `Date` from the [calendar date] (year, month and day).
    ///
    /// [calendar date]: NaiveDate#calendar-date
    ///
    /// # Panics
    ///
    /// Panics if the specified calendar day does not exist, on invalid values
    /// for `month` or `day`, or if `year` is out of range for `Date`.
    #[inline]
    #[must_use]
    pub const fn from_ymd(year: i32, month: u32, day: u32) -> Self {
        let Some(naive) = NaiveDate::from_ymd_opt(year, month, day) else {
            panic!("invalid or out-of-range date");
        };
        Date { naive, tz: Utc }
    }
}

impl<Tz> Date<Tz>
where
    Tz: TimeZone,
{
    #[inline]
    pub const fn naive_utc(&self) -> NaiveDate {
        self.naive
    }
}

impl<Tz> Datelike for Date<Tz>
where
    Tz: TimeZone,
{
    #[inline]
    fn year(&self) -> i32 {
        self.naive.year()
    }

    #[inline]
    fn month(&self) -> u32 {
        self.naive.month()
    }

    #[inline]
    fn month0(&self) -> u32 {
        self.naive.month0()
    }

    #[inline]
    fn day(&self) -> u32 {
        self.naive.day()
    }

    #[inline]
    fn day0(&self) -> u32 {
        self.naive.day0()
    }

    #[inline]
    fn ordinal(&self) -> u32 {
        self.naive.ordinal()
    }

    #[inline]
    fn ordinal0(&self) -> u32 {
        self.naive.ordinal0()
    }

    #[inline]
    fn weekday(&self) -> Weekday {
        self.naive.weekday()
    }

    #[inline]
    fn iso_week(&self) -> IsoWeek {
        self.naive.iso_week()
    }

    #[inline]
    fn with_year(&self, year: i32) -> Option<Self> {
        Some(Date {
            naive: self.naive.with_year(year)?,
            tz: self.tz.clone(),
        })
    }

    #[inline]
    fn with_month(&self, month: u32) -> Option<Self> {
        Some(Date {
            naive: self.naive.with_month(month)?,
            tz: self.tz.clone(),
        })
    }

    #[inline]
    fn with_month0(&self, month0: u32) -> Option<Self> {
        Some(Date {
            naive: self.naive.with_month0(month0)?,
            tz: self.tz.clone(),
        })
    }

    #[inline]
    fn with_day(&self, day: u32) -> Option<Self> {
        Some(Date {
            naive: self.naive.with_day(day)?,
            tz: self.tz.clone(),
        })
    }

    #[inline]
    fn with_day0(&self, day0: u32) -> Option<Self> {
        Some(Date {
            naive: self.naive.with_day0(day0)?,
            tz: self.tz.clone(),
        })
    }

    #[inline]
    fn with_ordinal(&self, ordinal: u32) -> Option<Self> {
        Some(Date {
            naive: self.naive.with_ordinal(ordinal)?,
            tz: self.tz.clone(),
        })
    }

    #[inline]
    fn with_ordinal0(&self, ordinal0: u32) -> Option<Self> {
        Some(Date {
            naive: self.naive.with_ordinal0(ordinal0)?,
            tz: self.tz.clone(),
        })
    }
}

impl Display for Date<Utc> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.naive, formatter)
    }
}

impl Debug for Date<Utc> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.naive, formatter)
    }
}

impl<Tz, Tz2> PartialEq<Date<Tz2>> for Date<Tz>
where
    Tz: TimeZone,
    Tz2: TimeZone,
{
    #[inline]
    fn eq(&self, other: &Date<Tz2>) -> bool {
        self.naive == other.naive
    }
}

impl<Tz> Eq for Date<Tz> where Tz: TimeZone {}

impl<Tz, Tz2> PartialOrd<Date<Tz2>> for Date<Tz>
where
    Tz: TimeZone,
    Tz2: TimeZone,
{
    #[inline]
    fn partial_cmp(&self, other: &Date<Tz2>) -> Option<Ordering> {
        self.naive.partial_cmp(&other.naive)
    }
}

impl<Tz> Ord for Date<Tz>
where
    Tz: TimeZone,
{
    #[inline]
    fn cmp(&self, other: &Date<Tz>) -> Ordering {
        self.naive.cmp(&other.naive)
    }
}

impl<Tz> Hash for Date<Tz>
where
    Tz: TimeZone,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.naive.hash(state);
    }
}

impl<Tz> Add<TimeDelta> for Date<Tz>
where
    Tz: TimeZone,
{
    type Output = Date<Tz>;

    #[inline]
    fn add(self, delta: TimeDelta) -> Date<Tz> {
        let date = self
            .naive
            .checked_add_signed(delta)
            .expect("`Date + TimeDelta` overflowed");
        Date {
            naive: date,
            tz: self.tz,
        }
    }
}

impl<Tz> AddAssign<TimeDelta> for Date<Tz>
where
    Tz: TimeZone,
{
    #[inline]
    fn add_assign(&mut self, delta: TimeDelta) {
        self.naive = self
            .naive
            .checked_add_signed(delta)
            .expect("`Date + TimeDelta` overflowed");
    }
}

impl<Tz> Sub<TimeDelta> for Date<Tz>
where
    Tz: TimeZone,
{
    type Output = Date<Tz>;

    #[inline]
    fn sub(self, delta: TimeDelta) -> Date<Tz> {
        let date = self
            .naive
            .checked_sub_signed(delta)
            .expect("`Date - TimeDelta` overflowed");
        Date {
            naive: date,
            tz: self.tz,
        }
    }
}

impl<Tz> SubAssign<TimeDelta> for Date<Tz>
where
    Tz: TimeZone,
{
    #[inline]
    fn sub_assign(&mut self, delta: TimeDelta) {
        self.naive = self
            .naive
            .checked_sub_signed(delta)
            .expect("`Date - TimeDelta` overflowed");
    }
}

impl<Tz> Sub<Date<Tz>> for Date<Tz>
where
    Tz: TimeZone,
{
    type Output = TimeDelta;

    #[inline]
    fn sub(self, rhs: Date<Tz>) -> TimeDelta {
        self.naive.signed_duration_since(rhs.naive)
    }
}

impl From<NaiveDate> for Date<Utc> {
    fn from(date: NaiveDate) -> Self {
        Date {
            naive: date,
            tz: Utc,
        }
    }
}

impl From<Date<Utc>> for NaiveDate {
    fn from(date: Date<Utc>) -> Self {
        date.naive
    }
}

struct CratesioDateVisitor;

impl<'de> Visitor<'de> for CratesioDateVisitor {
    type Value = Date<Utc>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("date in format 'YYYY-MM-DD'")
    }

    fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        'err: {
            if string.len() != 10 {
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
            return Ok(Date::from(naive_date));
        }
        Err(serde::de::Error::invalid_value(
            Unexpected::Str(string),
            &self,
        ))
    }
}

impl<'de> Deserialize<'de> for Date<Utc> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(CratesioDateVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::Date;
    use serde::de::value::Error;
    use serde::de::{Deserialize, IntoDeserializer};

    #[test]
    fn test_de() {
        let csv = "2020-01-01";
        let deserializer = IntoDeserializer::<Error>::into_deserializer;
        assert_eq!(
            Date::deserialize(deserializer(csv)).unwrap(),
            Date::from_ymd(2020, 1, 1),
        );
    }
}
