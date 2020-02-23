#[macro_use]
extern crate clap;

mod cli;
mod cmd;
mod io;
mod csv;

fn main() -> Result<(), &'static str>{
    let matches = cli::get_matches();

    match matches.subcommand_name() {
        Some("in") => cmd::inn::run(),
        Some("out") => cmd::out::run(),
        // clap takes care of unmatched subcommands
        _ => unreachable!()
    }
}
