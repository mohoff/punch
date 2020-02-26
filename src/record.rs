use std::fmt;

use chrono::{DateTime, Duration, Datelike};
use chrono::offset::{Local};
use serde::{Deserialize, Serialize};
use colored::*;

use crate::cli::Interval;

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
    pub fn is_terminated(&self) -> bool {
        self.end.is_some() && self.start <= self.end.unwrap()
    }
    pub fn bucket_key(&self, interval: Interval) -> u32 {
        match interval {
            Interval::Day => self.start.day(),
            Interval::Week => self.start.iso_week().week(),
            Interval::Month => self.start.month(),
            Interval::Year => self.start.year() as u32,
        }
    }
    pub fn display_aligned_with_records(&self, num_records: usize) -> String {
        let pad_left = num_records.to_string().len();

        format!(
            "{:0>pad$}: {} {}  {:<33} ({})",
            self.i.to_string().dimmed(),
            self.start.to_rfc3339(),
            "⟶".dimmed(),
            self.display_end(),
            self.display_duration().green(),
            pad = pad_left
        )
    }
    fn end(&self) -> DateTime<Local> {
        self.end.unwrap_or(Local::now())
    }
    fn display_end(&self) -> String {
        self.end.map_or("ongoing...".to_string(), |date| date.to_rfc3339())
    }
    fn duration(&self) -> Duration {
        self.end().signed_duration_since(self.start)
    }
    fn display_duration(&self) -> String {
        format_duration(self.duration())
    }
}

fn format_duration(d: Duration) -> String {
    // no built-in duration formatting available
    let h = d.num_hours();
    let min = d.num_minutes() - h * 60;
    let sec = d.num_seconds() - h * 3600 - min * 60;

    format!("{:0>#2}:{:0>#2}:{:0>#2}", h, min, sec)
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
        write!(f, "{}", self.display_aligned_with_records(0))
    }
}

pub struct RecordBucket(Vec<Record>, Interval);

impl fmt::Display for RecordBucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.name().bold().underline().to_string())?;
        writeln!(f, "{}", self.stats_formatted())?;

        for record in self.0.iter() {
            writeln!(f, "{}", record.display_aligned_with_records(self.size()))?;
        }

        Ok(())
    }
}

impl RecordBucket {
    pub fn new(interval: Interval) -> Self {
        RecordBucket(Vec::new(), interval)
    }
    pub fn add(&mut self, record: Record) {
        self.0.push(record)
    }
    pub fn name(&self) -> String {
        let date_str = (self.0)[0].start;

        let formatted = match self.1 {
            Interval::Day => date_str.format("%F (%A)"),
            Interval::Week => date_str.format("CW %U (%B %Y)"),
            Interval::Month => date_str.format("%B %Y"),
            Interval::Year => date_str.format("%Y"),
        };

        formatted.to_string()
    }
    pub fn stats_formatted(&self) -> String {
        format!(
            "{} ⏺️  - sum: {}, avg: {}",
            self.size().to_string().green(),
            format_duration(self.duration_sum()).green(),
            format_duration(self.duration_avg()).green()
        )
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

