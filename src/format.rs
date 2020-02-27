use crate::record::Record;

use chrono::{DateTime, Duration, Local};
use colored::*;

pub struct Formatter;

#[derive(Default)]
pub struct FormatRecordOptions {
    pub align_with_n_records: usize,
    pub precise: bool,
}

impl Formatter {
    pub fn format_record(r: &Record, opt: &FormatRecordOptions) -> String {
        let pad_index = opt.align_with_n_records.to_string().len();
        let pad_end = if opt.precise == true { 33 } else { 27 };

        let start = Self::format_datetime(&r.start, opt.precise);
        let end = r.end.map_or("ongoing...".to_string(), |date| Self::format_datetime(&date, opt.precise));
        let duration = Self::format_duration(r.duration()).bright_green();

        format!(
            "{:0>pad_index$}: {} {}  {:<pad_end$} ({})",
            r.i.to_string().dimmed(),
            start,
            "⟶".dimmed(),
            end,
            duration,
            pad_index = pad_index,
            pad_end = pad_end,
        )
    }

    pub fn format_duration(d: Duration) -> String {
        // no built-in duration formatting available
        let h = d.num_hours();
        let min = d.num_minutes() - h * 60;
        let sec = d.num_seconds() - h * 3600 - min * 60;
    
        format!("{:0>#2}:{:0>#2}:{:0>#2}", h, min, sec)
    }

    fn format_datetime(dt: &DateTime<Local>, precise: bool) -> String {
        match precise {
            true => dt.to_rfc3339(),
            false => dt.format("%F %T %Z").to_string(),
        }
    }
}

