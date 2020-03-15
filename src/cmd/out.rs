use crate::card::Card;
use crate::err::*;
use crate::time::Timestamp;
use colored::*;

pub fn run(note: Option<&str>) -> Result<()> {
    let card: Card = Default::default();

    let now = Timestamp::now();
    card.punch_out(now, note.map(String::from))?;

    print_success(now.format_with(&Default::default()));

    Ok(())
}

fn print_success(time: String) {
    let suffix = format!("at {}", time);
    println!("👊 out - {}", suffix.dimmed());
}
