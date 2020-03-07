use std::fmt;

use chrono::offset::Local;
use chrono::{DateTime, Datelike};
use serde::{Deserialize, Serialize};

use crate::cli::Interval;
use crate::duration::Duration;
use crate::format::Formatter;

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    pub i: usize,
    pub start: DateTime<Local>,
    #[serde(default)]
    pub end: Option<DateTime<Local>>,
    #[serde(default)]
    pub note: Option<String>,
}

impl Record {
    pub fn duration(&self) -> Duration {
        Duration::of_record(self)
    }
    pub fn is_terminated(&self) -> bool {
        self.end.is_some() && self.start <= self.end.unwrap()
    }
    pub fn bucket_key(&self, interval: Interval) -> u32 {
        match interval {
            Interval::Minute => (self.start.timestamp() / 60) as u32,
            Interval::Hour => (self.start.timestamp() / 3600) as u32,
            Interval::Day => self.start.day(),
            Interval::Week => self.start.iso_week().week(),
            Interval::Month => self.start.month(),
            Interval::Year => self.start.year() as u32,
        }
    }
}

impl From<(DateTime<Local>, usize, Option<String>)> for Record {
    fn from((timestamp, num_existing, note): (DateTime<Local>, usize, Option<String>)) -> Self {
        Record {
            i: num_existing,
            start: timestamp,
            end: None,
            note,
        }
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            Formatter::format_record(&self, &Default::default())
        )
    }
}
