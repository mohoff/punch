use std::fmt;
use chrono::{DateTime, Duration, Datelike};
use chrono::offset::{Local};
use colored::*;
use serde::{Deserialize};

use crate::cli::Interval;

#[derive(Debug, Deserialize)]
pub struct Record {
    i: usize,
    pub start: DateTime<Local>,
    #[serde(default)]
    end: Option<DateTime<Local>>,
    #[serde(default)]
    note: Option<String>,
}

impl Record {
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
    pub fn unwrap(r: Result<Self, csv::Error>) -> Self {
        r.unwrap()
    }
    pub fn group_by_interval(interval: Interval) -> Box<dyn Fn(Self) -> (u32, Self)> {
        Box::new(move |r: Self| {
            let key = match interval {
                Interval::Day => r.start.day(),
                Interval::Week => r.start.iso_week().week(),
                Interval::Month => r.start.month(),
                Interval::Year => r.start.year() as u32,
            };
    
            (key, r)
        })
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
}

fn format_duration(d: Duration) -> String {
    // no built-in duration formatting available
    let h = d.num_hours();
    let min = d.num_minutes() - h * 60;
    let sec = d.num_seconds() - h * 3600 - min * 60;

    format!("{:0>#2}:{:0>#2}:{:0>#2}", h, min, sec)
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.display_aligned_with_records(0))
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
}

