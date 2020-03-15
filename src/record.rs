use std::fmt;

use colored::*;
use serde::{Deserialize, Serialize};

use crate::format::RecordFormattingOptions;
use crate::time::{Duration, Interval, Timestamp};

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    pub i: usize,
    pub start: Timestamp,
    #[serde(default)]
    pub end: Option<Timestamp>,
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
        self.start.floor_to_interval_units(interval)
    }
    pub fn format_with(&self, opt: &RecordFormattingOptions) -> String {
        let pad_index = opt.align_with_n_records.to_string().len();
        let pad_end = match (opt.precise, opt.timezone) {
            (true, _) => 33,
            (_, true) => 26,
            (_, false) => 20,
        };

        let start = self.start.format_with(opt);
        let end = (self.end).map_or("ongoing...".to_string(), |date| date.format_with(opt));

        let duration = self.duration().round(&opt.rounding_opts);

        let duration = format!("({})", duration.format(&opt.rounding_opts).bright_green());
        let note = match &self.note {
            Some(n) => n.dimmed().to_string(),
            None => String::new(),
        };

        format!(
            "{:0>pad_index$}: {} {}  {:<pad_end$} {:<20} {}",
            self.i.to_string().dimmed(),
            start,
            "⟶".dimmed(),
            end,
            duration,
            note,
            pad_index = pad_index,
            pad_end = pad_end,
        )
    }
}

impl From<(Timestamp, usize, Option<String>)> for Record {
    fn from((timestamp, num_existing, note): (Timestamp, usize, Option<String>)) -> Self {
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
        write!(f, "{}", &self.format_with(&Default::default()))
    }
}
