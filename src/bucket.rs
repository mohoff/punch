use std::fmt;

use chrono::Duration;
use colored::*;

use crate::cli::Interval;
use crate::format::{Formatter, FormatRecordOptions};
use crate::record::Record;

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
                let rounded_sum = rounding.round_duration(&sum);
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

