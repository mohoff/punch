use std::fmt;

use chrono::{DateTime, Duration, Datelike};
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
            Interval::Minute => (self.start.timestamp() / 60) as u32,
            Interval::Hour => (self.start.timestamp() / 3600) as u32,
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
        write!(f, "{}", Formatter::format_record(&self, &Default::default()))
    }
}

pub struct RecordBucket(pub Vec<Record>, Interval, bool);

impl fmt::Display for RecordBucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let opts = FormatRecordOptions {
            align_with_n_records: self.size(),
            precise: self.2,
            rounding: None,
        };

        writeln!(f, "{}", self.name().bold().underline().to_string())?;
        writeln!(f, "{}", self.stats_formatted(&opts))?;

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
            Interval::Minute => {
                let next_date = date.checked_add_signed(Duration::minutes(1)).unwrap();
                
                let fst = date.format("%F (%A), %H:%M-");
                let snd = next_date.format("%H:%M (%Z)");

                format!("{}{}", fst, snd)
            },
            Interval::Hour => {
                let next_date = date.checked_add_signed(Duration::hours(1)).unwrap();

                let fst = date.format("%F (%A), %H:00-");
                let snd = next_date.format("%H:00 (%Z)");
            
                format!("{}{}", fst, snd)
            },
            Interval::Day => date.format("%F (%A)").to_string(),
            Interval::Week => date.format("CW %U (%B %Y)").to_string(),
            Interval::Month => date.format("%B %Y").to_string(),
            Interval::Year => date.format("%Y").to_string(),
        }
    }
    pub fn stats_formatted(&self, opt: &FormatRecordOptions) -> String {
        let num_punches = self.size().to_string().bright_green();

        let sum = self.duration_sum();
        let avg = self.duration_avg();

        match opt.rounding {
            Some(ref rounding) => {
                let rounded_sum = opt.rounding.as_ref().unwrap().round_duration(&sum);
                let sum_of_rounded = self.rounded_duration_sum(&opt);

                format!(
                    "{} ⏺️  - sum: {}, rounded sum: {}, sum of rounded: {}, avg: {}",
                    num_punches,
                    Formatter::format_duration(sum).bright_green(),
                    Formatter::format_duration(rounded_sum).bright_green(),
                    Formatter::format_duration(sum_of_rounded).bright_green(),
                    Formatter::format_duration(avg).bright_green(),
                )
            },
            None => {
                format!(
                    "{} ⏺️  - sum: {}, avg: {}",
                    num_punches,
                    Formatter::format_duration(sum).bright_green(),
                    Formatter::format_duration(avg).bright_green(),
                )
            }
        }
        
    }
    fn size(&self) -> usize {
        self.0.len()
    }
    fn rounded_duration_sum(&self, opt: &FormatRecordOptions) -> Duration {
        // Map to std::time::Duration and make use of its Sum trait implementation.
        let sum = self.0.iter()
            .map(|r| opt.rounding.as_ref().unwrap().round_duration(&r.duration())
                .to_std()
                .expect("Failed to convert duration")
            ).sum();

        Duration::from_std(sum).expect("Failed to compute duration sum")
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

