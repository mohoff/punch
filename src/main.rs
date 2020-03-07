#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;

mod bucket;
mod card;
mod cli;
mod cmd;
mod duration;
mod err;
mod format;
mod record;
mod round;

use std::convert::TryFrom;
use std::process;

use cli::Interval;
use err::*;
use round::RoundingOptions;

fn main() {
    match run() {
        Err(error) => {
            handle_error(&error);
            process::exit(1)
        }
        Ok(_) => process::exit(0),
    }
}

fn run() -> Result<()> {
    let matches = cli::get_matches();

    match matches.subcommand() {
        ("in", Some(in_matches)) => {
            let note = in_matches.value_of("note");
            cmd::inn::run(note)
        }
        ("out", Some(out_matches)) => {
            let note = out_matches.value_of("note");
            cmd::out::run(note)
        }
        ("show", Some(show_matches)) => {
            // using value_t! to get typed Interval instead of a string
            let interval =
                value_t!(show_matches.value_of("interval"), Interval).unwrap_or_else(|e| e.exit());
            let precise = show_matches.is_present("precise");
            // Use match instead of Option::map to allow use of ? operator
            let rounding = match show_matches.value_of("rounding") {
                Some(value) => RoundingOptions::try_from(value)?,
                None => RoundingOptions::default(),
            };

            cmd::show::run(interval, precise, rounding)
        }
        // clap takes care of unmatched subcommands
        _ => unreachable!(),
    }
}
