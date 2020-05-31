use crate::round::RoundingOptions;
use crate::time::Interval;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

pub fn get_matches<'a>() -> ArgMatches<'a> {
    let arg_note = Arg::with_name("note")
        .help("Attach a note to a punch")
        .takes_value(true)
        .empty_values(false)
        .index(1);

    #[allow(deprecated)]
    App::new("punch")
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .version_short("v")
        .settings(&[
            AppSettings::SubcommandRequiredElseHelp,
            AppSettings::GlobalVersion,
        ])
        .subcommand(
            SubCommand::with_name("status")
                .about("Show punch status"),
        )
        .subcommand(
            SubCommand::with_name("edit")
                .about("Edit a punch card"),
        )
        .subcommand(
            SubCommand::with_name("in")
                .about("Punch in - start tracking time")
                .arg(&arg_note),
        )
        .subcommand(
            SubCommand::with_name("out")
                .about("Punch out - stop tracking time")
                .arg(&arg_note),
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
                        // IMPROVE: passing Interval::Week here is favorable but clap requires a &str
                        .default_value("week"),
                )
                .arg(
                    Arg::with_name("precise").long("precise").short("p").help(
                        "Print timestamps precisely in RFC 3339 format (includes milliseconds)",
                    ),
                )
                .arg(
                    Arg::with_name("timezone")
                        .long("timezone")
                        .short("t")
                        .help("Print timestamps with timezones"),
                )
                .arg(
                    // A default value of "nearest,1min" is implemented through the Default trait instead of clap::Arg::default_value
                    Arg::with_name("rounding")
                        .long("round")
                        .short("r")
                        .takes_value(true)
                        .value_name("DIRECTION,GRANULARITY")
                        .validator(RoundingOptions::validate_str)
                        .help("Rounding string in format <DIRECTION,GRANULARITY> to specify rounding options for time durations. For example: nearest,1min (default); up,5min; down,1day")
                ),
        )
        .get_matches()
}
