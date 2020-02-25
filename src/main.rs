#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;

use std::process;

mod cli;
mod cmd;
mod io;
mod csv;
mod record;
mod err;

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
        ("in", _) => cmd::inn::run(),
        ("out", _) => cmd::out::run(),
        ("show", Some(show_matches)) => {
            // using value_t! to get typed Interval instead of a string
            let interval = value_t!(
                show_matches.value_of("interval"),
                Interval
            ).unwrap_or_else(|e| e.exit());

            cmd::show::run(interval)
        },
        // clap takes care of unmatched subcommands
        _ => unreachable!()
    }
}
