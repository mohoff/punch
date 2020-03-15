use std::convert::TryFrom;

use crate::err::*;
use crate::time::Interval;

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

#[derive(Debug, PartialEq, PartialOrd)]
pub struct RoundingGranularityInSeconds(pub usize);

impl Default for RoundingGranularityInSeconds {
    fn default() -> Self {
        RoundingGranularityInSeconds(60)
    }
}

impl TryFrom<&str> for RoundingGranularityInSeconds {
    type Error = Error;

    fn try_from(string: &str) -> Result<Self> {
        let (interval, amount): (String, String) = string.chars().partition(|c| c.is_alphabetic());

        let amount: usize = amount
            .parse()
            .chain_err(|| "Failed to parse rounding amount")?;
        let interval = Interval::try_from(interval.as_str())?;

        let mins = match interval {
            Interval::Second => amount,
            Interval::Minute => amount * 60,
            Interval::Hour => amount * 3600,
            Interval::Day => amount * 3600 * 24,
            Interval::Week => amount * 3600 * 24 * 7,
            _ => return Err(ErrorKind::InvalidTimeInterval.into()),
        };

        Ok(RoundingGranularityInSeconds(mins))
    }
}

#[derive(Debug)]
pub struct RoundingOptions {
    pub direction: RoundingDirection,
    pub granularity: RoundingGranularityInSeconds,
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
            granularity: RoundingGranularityInSeconds::try_from(elements[1])?,
        })
    }
}

impl RoundingOptions {
    // This is a utility for clap::Arg::validator used in cli.rs
    pub fn validate_str(input: String) -> std::result::Result<(), String> {
        let elements = input.split(',').collect::<Vec<_>>();

        if elements.len() != 2 {
            return Err(format!(
                "Failed to parse <ROUNDING> argument {:?}. Expected <ROUNDING> to be of form '<DIRECTION>,<INTERVAL>'. For example: up,5min", elements
            ));
        }

        let _ = RoundingOptions {
            direction: RoundingDirection::try_from(elements[0])
                .map_err(|_|
                    format!("Failed to parse rounding DIRECTION {:?}. Expected one of the following: \"nearest\" (\"n\"), \"up\" (\"u\"), or \"down\" (\"d\")", elements[0])
                )?,
            granularity: RoundingGranularityInSeconds::try_from(elements[1])
                .map_err(|_|
                    format!("Failed to parse rounding GRANULARITY {:?}. Expected a time specification like \"5min\", \"1h\", \"3days\", etc.", elements[1])
                )?,
        };

        Ok(())
    }
}
