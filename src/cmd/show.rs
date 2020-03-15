use crate::card::Card;
use crate::err::*;
use crate::format::CardFormattingOptions;

pub fn run(opts: CardFormattingOptions) -> Result<()> {
    let card: Card = Default::default();

    card.display_with(opts)
}
