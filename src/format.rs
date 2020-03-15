use crate::round::RoundingOptions;
use crate::time::Interval;

#[derive(Default)]
pub struct CardFormattingOptions {
    pub interval: Interval,
    pub record_opts: RecordFormattingOptions,
}

pub struct RecordFormattingOptions {
    pub align_with_n_records: usize,
    pub precise: bool,
    pub timezone: bool,
    pub rounding_opts: RoundingOptions,
}

impl Default for RecordFormattingOptions {
    fn default() -> Self {
        RecordFormattingOptions {
            align_with_n_records: 0,
            precise: false,
            timezone: true,
            rounding_opts: Default::default(),
        }
    }
}
