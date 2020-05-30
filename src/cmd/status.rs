use crate::card::Card;
use crate::err::*;

pub fn run() -> Result<()> {
    let card: Card = Default::default();

    println!("{}", card.status()?);

    Ok(())
}
