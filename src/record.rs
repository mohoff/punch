use std::fmt;
use chrono::{DateTime, Duration};
use chrono::offset::{Local};
use colored::*;
use serde::{Deserialize};

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
        write!(
            f,
            "{}: {} {} {:<33} ({})",
            self.i.to_string().dimmed(),
            self.start.to_rfc3339(),
            "->".dimmed(),
            self.display_end(),
            self.display_duration().green()
        )
    }
}

pub enum BucketType {
    DAY,
    WEEK,
    MONTH,
    YEAR,
}

pub struct RecordBucket(Vec<Record>);

impl RecordBucket {
    pub fn new() -> Self {
        RecordBucket(Vec::new())
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

    pub fn name(&self, bucket_type: &BucketType) -> String {
        let date_str = (self.0)[0].start;

        match bucket_type {
            BucketType::DAY => date_str.format("%F (%A)").to_string(),
            BucketType::WEEK => date_str.format("%U (%B %Y)").to_string(),
            BucketType::MONTH => date_str.format("%B %Y").to_string(),
            BucketType::YEAR => date_str.format("%Y").to_string(),
        }
    }
    pub fn name_formatted(&self, bucket_type: &BucketType) -> String {
        self.name(bucket_type).bold().underline().to_string()
    }

    pub fn stats_formatted(&self) -> String {
        format!(
            "sum: {}, avg: {}",
            format_duration(self.duration_sum()).green(),
            format_duration(self.duration_avg()).green()
        )
    }

    pub fn records_formatted(&self) -> Vec<String> {
        self.0.iter().map(|r| r.to_string()).collect::<Vec<_>>()
    }
}

