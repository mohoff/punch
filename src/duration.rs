use chrono::offset::Local;
use chrono::{self};
use std::convert::Into;
use std::iter::Sum;

use crate::record::Record;
use crate::round::{RoundingDirection, RoundingOptions};

pub struct Duration(chrono::Duration);

impl From<chrono::Duration> for Duration {
    fn from(d: chrono::Duration) -> Self {
        Self(d)
    }
}
impl Into<chrono::Duration> for Duration {
    fn into(self) -> chrono::Duration {
        self.0
    }
}

pub trait Mean<A = Self> {
    fn mean<I: Iterator<Item = A>>(iter: I) -> Self;
}

impl Mean for Duration {
    fn mean<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let mut num_items = 0;
        let sum = iter.fold(chrono::Duration::zero(), |acc, x| {
            num_items += 1;
            acc + x.0
        });

        Self(sum / num_items)
    }
}

impl Sum for Duration {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let sum = iter.fold(chrono::Duration::zero(), |acc, x| acc + x.0);

        Self(sum)
    }
}

impl Duration {
    fn from_minutes(minutes: usize) -> Self {
        Self(chrono::Duration::minutes(minutes as i64))
    }
    pub fn of_record(r: &Record) -> Self {
        let end = r.end.clone().unwrap_or_else(Local::now);

        Self(end.signed_duration_since(r.start))
    }
    fn in_minutes(&self) -> usize {
        self.0.num_minutes() as usize
    }
    pub fn one_minute() -> Self {
        Self(chrono::Duration::minutes(1))
    }
    pub fn one_hour() -> Self {
        Self(chrono::Duration::hours(1))
    }
    pub fn round(&self, opt: &RoundingOptions) -> Self {
        let exact_minutes = self.in_minutes();

        if exact_minutes % opt.granularity.0 == 0 {
            return Self::from_minutes(exact_minutes);
        }

        let exceeding_lower_by = exact_minutes % opt.granularity.0;

        let lower = exact_minutes - exceeding_lower_by;
        let upper = lower + opt.granularity.0;
        let exact_median = opt.granularity.0 as f64 / 2_f64;

        let rounded_mins = match opt.direction {
            RoundingDirection::Up => upper,
            RoundingDirection::Down => lower,
            RoundingDirection::Nearest => {
                if exceeding_lower_by as f64 >= exact_median {
                    upper
                } else {
                    lower
                }
            }
        };

        Self::from_minutes(rounded_mins)
    }
    pub fn format(&self) -> String {
        // no built-in duration formatting available
        let h = self.0.num_hours();
        let min = self.0.num_minutes() - h * 60;
        let sec = self.0.num_seconds() - h * 3600 - min * 60;

        format!("{:0>#2}:{:0>#2}:{:0>#2}", h, min, sec)
    }
}
