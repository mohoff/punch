use crate::card::Card;
use crate::cli::Interval;
use crate::round::Rounding;
use crate::err::*;

pub fn run(interval: Interval, precise: bool, rounding: Option<Rounding>) -> Result<()> {
    let card: Card = Default::default();

    card.display_with(interval, precise, rounding)
}