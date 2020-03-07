use crate::bucket::RecordBucket;
use crate::record::Record;
use crate::round::RoundingOptions;

use chrono::{DateTime, Local};
use colored::*;

pub struct Formatter;

#[derive(Default)]
pub struct FormatRecordOptions {
    pub align_with_n_records: usize,
    pub precise: bool,
    pub rounding: RoundingOptions,
}

impl Formatter {
    pub fn format_bucket(b: &RecordBucket, opt: &FormatRecordOptions) -> String {
        let records =
            b.0.iter()
                .map(|r| Formatter::format_record(r, &opt))
                .collect::<Vec<_>>()
                .join("\n");

        format!(
            "{}\n{}\n{}\n",
            b.name().bold().underline().to_string(),
            b.stats_formatted(&opt),
            records
        )
    }

    pub fn format_record(r: &Record, opt: &FormatRecordOptions) -> String {
        let pad_index = opt.align_with_n_records.to_string().len();
        let pad_end = if opt.precise { 33 } else { 27 };

        let start = Self::format_datetime(&r.start, opt.precise);
        let end = r.end.map_or("ongoing...".to_string(), |date| {
            Self::format_datetime(&date, opt.precise)
        });

        let duration = r.duration().round(&opt.rounding);

        let duration = format!("({})", duration.format().bright_green());
        let note = match &r.note {
            Some(n) => n.dimmed().to_string(),
            None => String::new(),
        };

        format!(
            "{:0>pad_index$}: {} {}  {:<pad_end$} {:<20} {}",
            r.i.to_string().dimmed(),
            start,
            "⟶".dimmed(),
            end,
            duration,
            note,
            pad_index = pad_index,
            pad_end = pad_end,
        )
    }

    fn format_datetime(dt: &DateTime<Local>, precise: bool) -> String {
        if precise {
            dt.to_rfc3339()
        } else {
            dt.format("%F %T %Z").to_string()
        }
    }
}
