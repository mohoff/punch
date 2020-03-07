use crate::card::Card;
use crate::cli::Interval;
use crate::err::*;
use crate::round::RoundingOptions;

pub fn run(interval: Interval, precise: bool, rounding: RoundingOptions) -> Result<()> {
    let card: Card = Default::default();

    card.display_with(interval, precise, rounding)
}
