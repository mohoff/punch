use std::convert::TryFrom;

use crate::cli::Interval;
use crate::err::*;

#[derive(Debug)]
pub enum RoundingDirection {
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
pub struct RoundingGranularityInMinutes(pub usize);

impl Default for RoundingGranularityInMinutes {
    fn default() -> Self {
        RoundingGranularityInMinutes(0)
    }
}

impl TryFrom<&str> for RoundingGranularityInMinutes {
    type Error = Error;

    fn try_from(string: &str) -> Result<Self> {
        let (interval, amount): (String, String) = string.chars().partition(|c| c.is_alphabetic());

        let amount: usize = amount
            .parse()
            .chain_err(|| "Failed to parse rounding amount")?;
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
pub struct RoundingOptions {
    pub direction: RoundingDirection,
    pub granularity: RoundingGranularityInMinutes,
}

impl Default for RoundingOptions {
    fn default() -> Self {
        RoundingOptions {
            direction: RoundingDirection::Nearest,
            granularity: Default::default(),
        }
    }
}

impl TryFrom<&str> for RoundingOptions {
    type Error = Error;

    fn try_from(rounding_str: &str) -> Result<Self> {
        let elements = rounding_str.split(',').collect::<Vec<_>>();

        if elements.len() != 2 {
            return Err(ErrorKind::InvalidTimeInterval.into());
        }

        Ok(RoundingOptions {
            direction: RoundingDirection::try_from(elements[0])?,
            granularity: RoundingGranularityInMinutes::try_from(elements[1])?,
        })
    }
}
