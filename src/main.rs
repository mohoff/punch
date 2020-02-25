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

    match matches.subcommand_name() {
        Some("in") => cmd::inn::run(),
        Some("out") => cmd::out::run(),
        Some("show") => cmd::show::run(),
        // clap takes care of unmatched subcommands
        _ => unreachable!()
    }
}
