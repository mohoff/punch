use clap::{App, SubCommand, ArgMatches};

pub fn get_matches<'a>() -> ArgMatches<'a> {
    App::new("timetracker")
        .version(crate_version!())
        .author("Moritz Hoffmann <mohoff@web.de>")
        .subcommand(
            SubCommand::with_name("in")
                .about("Start tracking time")
        )
        .subcommand(
            SubCommand::with_name("out")
                .about("Stop tracking time")
        )
        .get_matches()
}