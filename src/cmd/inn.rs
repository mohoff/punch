use crate::card::Card;
use crate::err::*;
use chrono::{Local};
use colored::*;

pub fn run(note: Option<&str>) -> Result<()> {
    let card: Card = Default::default();

    let now = Local::now();
    card.punch_in(now, note)?;

    print_success(now.to_rfc3339());

    Ok(())
}

fn print_success(time: String) {
    let suffix = format!("at {}", time);
    println!("👊 in - {}", suffix.dimmed());
}