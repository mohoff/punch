use chrono::offset::Local;
use chrono::{self};
use colored::*;
use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use std::cmp::Ordering;
use std::convert::Into;
use std::convert::TryFrom;
use std::iter::Sum;
use std::ops::Add;

use crate::err::*;
use crate::format::RecordFormattingOptions;
use crate::record::Record;
use crate::round::{RoundingDirection, RoundingGranularityInSeconds, RoundingOptions};

#[derive(Debug)]
pub struct Duration(chrono::Duration);

impl From<chrono::Duration> for Duration {
    fn from(d: chrono::Duration) -> Self {
        Self(d)
    }
}
impl Into<chrono::Duration> for Duration {
    fn into(self) -> chrono::Duration {
        self.0
    }
}

pub trait Mean<A = Self> {
    fn mean<I: Iterator<Item = A>>(iter: I) -> Self;
}

impl Mean for Duration {
    fn mean<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let mut num_items = 0;
        let sum = iter.fold(chrono::Duration::zero(), |acc, x| {
            num_items += 1;
            acc + x.0
        });

        Self(sum / num_items)
    }
}

impl Sum for Duration {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let sum = iter.fold(chrono::Duration::zero(), |acc, x| acc + x.0);

        Self(sum)
    }
}

impl Duration {
    fn from_seconds(seconds: usize) -> Self {
        Self(chrono::Duration::seconds(seconds as i64))
    }
    pub fn of_record(r: &Record) -> Self {
        let end = r.end.clone().unwrap_or_else(Timestamp::now);

        Self::between(r.start, end)
    }
    fn between(from: Timestamp, to: Timestamp) -> Self {
        Self(to.0.signed_duration_since(from.0))
    }
    fn in_seconds(&self) -> usize {
        self.0.num_seconds() as usize
    }
    pub fn one_minute() -> Self {
        Self(chrono::Duration::minutes(1))
    }
    pub fn one_hour() -> Self {
        Self(chrono::Duration::hours(1))
    }
    pub fn round(&self, opt: &RoundingOptions) -> Self {
        let exact_seconds = self.in_seconds();

        if exact_seconds % opt.granularity.0 == 0 {
            return Self::from_seconds(exact_seconds);
        }

        let exceeding_lower_by = exact_seconds % opt.granularity.0;

        let lower = exact_seconds - exceeding_lower_by;
        let upper = lower + opt.granularity.0;
        let exact_median = opt.granularity.0 as f64 / 2_f64;

        let rounded_seconds = match opt.direction {
            RoundingDirection::Up => upper,
            RoundingDirection::Down => lower,
            RoundingDirection::Nearest => {
                if exceeding_lower_by as f64 >= exact_median {
                    upper
                } else {
                    lower
                }
            }
        };

        Self::from_seconds(rounded_seconds)
    }
    pub fn format(&self, opt: &RoundingOptions) -> String {
        // no built-in duration formatting available
        let h = self.0.num_hours();
        let min = self.0.num_minutes() - h * 60;

        if opt.granularity < RoundingGranularityInSeconds(60) {
            let sec = self.0.num_seconds() - h * 3600 - min * 60;
            format!("{:0>#2}:{:0>#2}:{:0>#2}h", h, min, sec)
        } else {
            format!("{:0>#2}:{:0>#2}h", h, min)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Timestamp(chrono::DateTime<Local>);

impl PartialEq for Timestamp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Add<Duration> for Timestamp {
    type Output = Self;

    fn add(self, other: Duration) -> Self {
        Self(self.0 + Into::<chrono::Duration>::into(other))
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let dt = chrono::DateTime::deserialize(deserializer)?;

        Ok(Self(dt))
    }
}

impl Timestamp {
    pub fn format(&self, format_str: &str) -> String {
        self.0.format(format_str).to_string()
    }
    pub fn format_with(&self, opts: &RecordFormattingOptions) -> String {
        if opts.precise {
            self.0.to_rfc3339()
        } else if opts.timezone {
            format!(
                "{} {}",
                self.0.format("%F %T"),
                self.0.format("%Z").to_string().dimmed()
            )
        } else {
            self.0.format("%F %T").to_string()
        }
    }
    pub fn now() -> Self {
        Timestamp(Local::now())
    }
    pub fn floor_to_interval_units(&self, interval: Interval) -> u32 {
        use chrono::Datelike;

        let year = self.0.year() as u32;
        match interval {
            Interval::Second => self.0.timestamp() as u32,
            Interval::Minute => (self.0.timestamp() / 60) as u32,
            Interval::Hour => (self.0.timestamp() / 3600) as u32,
            Interval::Day => year * 10000 + self.0.month() * 100 + self.0.day(),
            Interval::Week => year * 100 + self.0.iso_week().week(),
            Interval::Month => year * 100 + self.0.month(),
            Interval::Year => year,
        }
    }
}

arg_enum! {
    #[derive(Clone, Copy, Debug)]
    pub enum Interval {
        Second,
        Minute,
        Hour,
        Day,
        Week,
        Month,
        Year,
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval::Day
    }
}

impl TryFrom<&str> for Interval {
    type Error = Error;

    fn try_from(string: &str) -> Result<Self> {
        match string {
            "min" | "m" => Ok(Interval::Minute),
            "hour" | "h" => Ok(Interval::Hour),
            "day" | "d" => Ok(Interval::Day),
            "week" | "w" => Ok(Interval::Week),
            _ => Err(ErrorKind::InvalidTimeInterval.into()),
        }
    }
}
