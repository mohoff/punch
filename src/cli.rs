use clap::{App, SubCommand, AppSettings, Arg, ArgMatches};

arg_enum!{
    #[derive(Clone, Copy, Debug)]
    pub enum Interval {
        Hour,
        Day,
        Week,
        Month,
        Year,
    }
}

pub fn get_matches<'a>() -> ArgMatches<'a> {
    let arg_note = Arg::with_name("note")
        .help("Attach a note to a punch")
        .takes_value(true)
        .empty_values(false)
        .index(1);

    App::new("punch")
        .version(crate_version!())
        .version_short("v")
        .author("Moritz Hoffmann <mohoff@web.de>")
        .settings(&[
            AppSettings::SubcommandRequiredElseHelp,
            AppSettings::GlobalVersion
        ])
        .subcommand(
            SubCommand::with_name("in")
                .about("Punch in - start tracking time")
                .arg(&arg_note)
        )
        .subcommand(
            SubCommand::with_name("out")
                .about("Punch out - stop tracking time")
                .arg(&arg_note)
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
                        // IMPROVE: smth Interval::Week would be cleaner but &str is expected here
                        .default_value("week")
                )
                .arg(
                    Arg::with_name("precise")
                        .short("p")
                        .help("Precisely print timestamps in RFC 3339 format (includes milliseconds)")
                )
        )
        .get_matches()
}