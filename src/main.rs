#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;

mod cli;
mod cmd;
mod card;
mod record;
mod format;
mod err;

use std::process;

use err::*;
use cli::Interval;

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
        },
        ("out", Some(out_matches)) => {
            let note = out_matches.value_of("note");
            cmd::out::run(note)
        },
        ("show", Some(show_matches)) => {
            // using value_t! to get typed Interval instead of a string
            let interval = value_t!(
                show_matches.value_of("interval"),
                Interval
            ).unwrap_or_else(|e| e.exit());
            let precise = show_matches.is_present("precise");

            cmd::show::run(interval, precise)
        },
        // clap takes care of unmatched subcommands
        _ => unreachable!()
    }
}
