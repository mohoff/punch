use std::fmt;

use chrono::{DateTime, Duration, Datelike, Timelike};
use chrono::offset::{Local};
use serde::{Deserialize, Serialize};
use colored::*;

use crate::cli::Interval;
use crate::format::{Formatter, FormatRecordOptions};

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
        self.end().signed_duration_since(self.start)
    }
    pub fn is_terminated(&self) -> bool {
        self.end.is_some() && self.start <= self.end.unwrap()
    }
    pub fn bucket_key(&self, interval: Interval) -> u32 {
        match interval {
            Interval::Hour => self.start.naive_utc().hour(),
            Interval::Day => self.start.day(),
            Interval::Week => self.start.iso_week().week(),
            Interval::Month => self.start.month(),
            Interval::Year => self.start.year() as u32,
        }
    }
    fn end(&self) -> DateTime<Local> {
        self.end.unwrap_or(Local::now())
    }
}

impl From<(DateTime<Local>, usize)> for Record {
    fn from((timestamp, num_existing): (DateTime<Local>, usize)) -> Self {
        Record {
            i: num_existing,
            start: timestamp,
            end: None,
            note: None,
        }
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Formatter::format_record(&self, &Default::default()))
    }
}

pub struct RecordBucket(Vec<Record>, Interval, bool);

impl fmt::Display for RecordBucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.name().bold().underline().to_string())?;
        writeln!(f, "{}", self.stats_formatted())?;

        let opts = FormatRecordOptions {
            align_with_n_records: self.size(),
            precise: self.2,
        };

        for record in self.0.iter() {
            writeln!(f, "{}", Formatter::format_record(record, &opts))?;
        }

        Ok(())
    }
}

impl RecordBucket {
    pub fn new(interval: Interval, precise: bool) -> Self {
        RecordBucket(Vec::new(), interval, precise)
    }
    pub fn add(&mut self, record: Record) {
        self.0.push(record)
    }
    pub fn name(&self) -> String {
        let date = (self.0)[0].start;

        match self.1 {
            Interval::Hour => {
                let next_hour = date.format("%H")
                .to_string()
                .parse::<u8>()
                .expect("Failed to parse hour")
                + 1 % 24;
                let next_hour = match next_hour {
                    0 | 1 => format!("{}:00 {}", next_hour, "(next day)".dimmed()),
                    _ => format!("{}:00", next_hour),
                };

                let prefix = date.format("%F (%A), %H:00-");
                let suffix = date.format(" (%Z)");
            
                format!("{}{}{}", prefix, next_hour, suffix)
            },
            Interval::Day => date.format("%F (%A)").to_string(),
            Interval::Week => date.format("CW %U (%B %Y)").to_string(),
            Interval::Month => date.format("%B %Y").to_string(),
            Interval::Year => date.format("%Y").to_string(),
        }
    }
    pub fn stats_formatted(&self) -> String {
        let num_punches = self.size().to_string().bright_green();
        let sum = Formatter::format_duration(self.duration_sum()).bright_green();
        let avg = Formatter::format_duration(self.duration_avg()).bright_green();
    
        format!("{} ⏺️  - sum: {}, avg: {}", num_punches, sum, avg)
    }
    fn size(&self) -> usize {
        self.0.len()
    }
    fn duration_sum(&self) -> Duration {
        // Map to std::time::Duration and make use of its Sum trait implementation.
        let sum = self.0.iter()
            .map(|r| r.duration()
                .to_std()
                .expect("Failed to convert duration")
            ).sum();

        Duration::from_std(sum).expect("Failed to compute duration sum")
    }
    fn duration_avg(&self) -> Duration {
        let sum = self.duration_sum();

        sum / (self.size() as i32)
    }
}

