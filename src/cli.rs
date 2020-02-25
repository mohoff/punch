use clap::{App, SubCommand, Arg, ArgMatches};

arg_enum!{
    #[derive(Clone, Copy, Debug)]
    pub enum Interval {
        Day,
        Week,
        Month,
        Year,
    }
}

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
                .arg(
                    Arg::with_name("interval")
                        .help("The time interval at which records should be grouped together")
                        .index(1)
                        .case_insensitive(true)
                        .possible_values(&Interval::variants())
                        .default_value("week") // IMPROVE: smth Interval::Week would be cleaner but &str is expected here
                )
        )
        .get_matches()
}