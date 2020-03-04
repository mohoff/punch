use std::convert::TryFrom;

use chrono::{Duration};

use crate::cli::Interval;
use crate::err::*;

#[derive(Debug)]
enum RoundingDirection {
    Down,
    Up,
    Nearest,
}

impl TryFrom<&str> for RoundingDirection {
    type Error = Error;

    fn try_from(string: &str) -> Result<Self> {
        match string.to_lowercase().as_str() {
            "down" | "d" => Ok(RoundingDirection::Down),
            "up" | "u" => Ok(RoundingDirection::Up),
            "nearest" | "n" => Ok(RoundingDirection::Nearest),
            _ => Err(ErrorKind::InvalidRoundingDirection.into()),
        }
    }
}

#[derive(Debug)]
struct RoundingGranularityInMinutes(usize);

impl TryFrom<&str> for RoundingGranularityInMinutes {
    type Error = Error;

    fn try_from(string: &str) -> Result<Self> {
        let (interval, amount): (String, String) = string.chars()
            .partition(|c| c.is_alphabetic());
    
        let amount: usize = amount.parse().chain_err(|| "Failed to parse rounding amount")?;
        let interval = Interval::try_from(interval.as_str())?;
        
        let mins = match interval {
            Interval::Minute => amount,
            Interval::Hour => amount * 60,
            Interval::Day => amount * 60 * 24,
            Interval::Week => amount * 60 * 24 * 7,
            _ => return Err(ErrorKind::InvalidTimeInterval.into()),
        };

        Ok(RoundingGranularityInMinutes(mins))
    }
}

#[derive(Debug)]
pub struct Rounding {
    direction: RoundingDirection,
    granularity: RoundingGranularityInMinutes,
}

impl TryFrom<&str> for Rounding {
    type Error = Error;

    fn try_from(rounding_str: &str) -> Result<Self> {
        let elements = rounding_str.split(',').collect::<Vec<_>>();

        if elements.len() != 2 {
            return Err(ErrorKind::InvalidTimeInterval.into());
        }

        Ok(Rounding {
            direction: RoundingDirection::try_from(elements[0])?,
            granularity: RoundingGranularityInMinutes::try_from(elements[1])?,
        })
    }
}

impl Rounding {
    pub fn round_duration(&self, d: &Duration) -> Duration {
        let exact_minutes = d.num_minutes() as usize;

        if exact_minutes % self.granularity.0 == 0 {
            return Duration::minutes(exact_minutes as i64);
        }

        let exceeding_lower_by = exact_minutes % self.granularity.0; 

        let lower = exact_minutes - exceeding_lower_by;
        let upper = lower + self.granularity.0;
        let exact_median = self.granularity.0 as f64 / 2 as f64;

        let rounded_mins = match self.direction {
            RoundingDirection::Up => upper,
            RoundingDirection::Down => lower,
            RoundingDirection::Nearest => if exceeding_lower_by as f64 >= exact_median { upper } else { lower },
        };

        Duration::minutes(rounded_mins as i64)
    }
}
