use crate::card::Card;
use crate::cli::Interval;
use crate::err::*;

pub fn run(interval: Interval) -> Result<()> {
    let card: Card = Default::default();

    card.display_with(interval)
}