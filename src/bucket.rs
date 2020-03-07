use std::fmt;

use chrono::{self, DateTime, Local};
use colored::*;

use crate::cli::Interval;
use crate::duration::Duration;
use crate::duration::Mean;
use crate::format::{FormatRecordOptions, Formatter};
use crate::record::Record;

pub struct RecordBucket(pub Vec<Record>, Interval, bool);

impl fmt::Display for RecordBucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let opts = FormatRecordOptions {
            align_with_n_records: self.size(),
            precise: self.2,
            rounding: Default::default(),
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
                let next_date: DateTime<Local> =
                    date + Into::<chrono::Duration>::into(Duration::one_minute());

                let fst = date.format("%F (%A), %H:%M-");
                let snd = next_date.format("%H:%M (%Z)");

                format!("{}{}", fst, snd)
            }
            Interval::Hour => {
                let d: chrono::Duration = Duration::one_hour().into();
                let next_date: DateTime<Local> = date + d;

                let fst = date.format("%F (%A), %H:00-");
                let snd = next_date.format("%H:00 (%Z)");

                format!("{}{}", fst, snd)
            }
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

        let rounded_sum = sum.round(&opt.rounding);
        let sum_of_rounded = self.rounded_duration_sum(&opt);

        format!(
            "{} ⏺️  - sum: {}, rounded sum: {}, sum of rounded: {}, avg: {}",
            num_punches,
            sum.format().bright_green(),
            rounded_sum.format().bright_green(),
            sum_of_rounded.format().bright_green(),
            avg.format().bright_green(),
        )
    }
    fn size(&self) -> usize {
        self.0.len()
    }
    fn rounded_duration_sum(&self, opt: &FormatRecordOptions) -> Duration {
        self.duration_sum().round(&opt.rounding)
    }
    fn duration_sum(&self) -> Duration {
        self.0.iter().map(|r| r.duration()).sum::<Duration>()
    }
    fn duration_avg(&self) -> Duration {
        Mean::mean(self.0.iter().map(|r| r.duration()))
    }
}
