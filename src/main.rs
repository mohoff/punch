#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;

mod bucket;
mod card;
mod cli;
mod cmd;
mod err;
mod format;
mod record;
mod round;
mod time;

use std::convert::TryFrom;
use std::process;

use err::*;
use format::{CardFormattingOptions, RecordFormattingOptions};
use round::RoundingOptions;
use time::Interval;

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
        ("status", _) => {
            cmd::status::run()
        }
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
            let timezone = show_matches.is_present("timezone");
            let rounding = show_matches
                .value_of("rounding")
                .map_or(Ok(Default::default()), RoundingOptions::try_from)?;

            let opts = CardFormattingOptions {
                interval,
                record_opts: RecordFormattingOptions {
                    rounding_opts: rounding,
                    precise,
                    timezone,
                    ..Default::default()
                },
            };

            cmd::show::run(opts)
        }
        // clap takes care of unmatched subcommands
        _ => unreachable!(),
    }
}
