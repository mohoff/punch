use clap::{App, SubCommand, ArgMatches};

pub fn get_matches<'a>() -> ArgMatches<'a> {
    App::new("punch")
        .version(crate_version!())
        .author("Moritz Hoffmann <mohoff@web.de>")
        .subcommand(
            SubCommand::with_name("in")
                .about("Punch in - start tracking time")
        )
        .subcommand(
            SubCommand::with_name("out")
                .about("Punch out - stop tracking time")
        )
        .subcommand(
            SubCommand::with_name("show")
                .about("Show details of a punch card")
        )
        .get_matches()
}