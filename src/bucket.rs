use std::fmt;

use colored::*;

use crate::format::RecordFormattingOptions;
use crate::record::Record;
use crate::time::Mean;
use crate::time::{Duration, Interval, Timestamp};

pub struct RecordBucket(pub Vec<Record>, Interval, bool);

impl fmt::Display for RecordBucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let opts = RecordFormattingOptions {
            align_with_n_records: self.size(),
            precise: self.2,
            timezone: true,
            rounding_opts: Default::default(),
        };

        writeln!(f, "{}", self.name().bold().underline().to_string())?;
        writeln!(f, "{}", self.format_stats_with(&opts))?;

        for record in self.0.iter() {
            writeln!(f, "{}", record.format_with(&opts))?;
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
            Interval::Second => unreachable!(),
            Interval::Minute => {
                let next_date: Timestamp = date + Duration::one_minute();

                let fst = date.format("%F (%A), %H:%M-");
                let snd = next_date.format("%H:%M (%Z)");

                format!("{}{}", fst, snd)
            }
            Interval::Hour => {
                let next_date: Timestamp = date + Duration::one_hour();

                let fst = date.format("%F (%A), %H:00-");
                let snd = next_date.format("%H:00 (%Z)");

                format!("{}{}", fst, snd)
            }
            Interval::Day => date.format("%F (%A)"),
            Interval::Week => date.format("CW %U (%B %Y)"),
            Interval::Month => date.format("%B %Y"),
            Interval::Year => date.format("%Y"),
        }
    }
    fn size(&self) -> usize {
        self.0.len()
    }
    fn rounded_duration_sum(&self, opt: &RecordFormattingOptions) -> Duration {
        self.duration_sum().round(&opt.rounding_opts)
    }
    fn duration_sum(&self) -> Duration {
        self.0.iter().map(|r| r.duration()).sum::<Duration>()
    }
    fn duration_avg(&self) -> Duration {
        Mean::mean(self.0.iter().map(|r| r.duration()))
    }
    pub fn display_with(&self, opts: &RecordFormattingOptions) {
        let records = (self.0)
            .iter()
            .rev()
            .map(|r| r.format_with(&opts))
            .collect::<Vec<_>>()
            .join("\n");

        println!(
            "{}\n{}\n{}\n",
            self.name().bold().underline().to_string(),
            self.format_stats_with(&opts),
            records
        )
    }
    pub fn format_stats_with(&self, opt: &RecordFormattingOptions) -> String {
        let num_punches = self.size().to_string().bright_green();

        let sum = self.duration_sum();
        let avg = self.duration_avg();

        let rounded_sum = sum.round(&opt.rounding_opts);
        let sum_of_rounded = self.rounded_duration_sum(&opt);

        format!(
            "{} ⏺️  - sum: {}, rounded sum: {}, sum of rounded: {}, avg: {}",
            num_punches,
            sum.format(&opt.rounding_opts).bright_green(),
            rounded_sum.format(&opt.rounding_opts).bright_green(),
            sum_of_rounded.format(&opt.rounding_opts).bright_green(),
            avg.format(&opt.rounding_opts).bright_green(),
        )
    }
}
