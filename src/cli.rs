use std::convert::TryFrom;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use crate::err::*;

arg_enum! {
    #[derive(Clone, Copy, Debug)]
    pub enum Interval {
        Minute,
        Hour,
        Day,
        Week,
        Month,
        Year,
    }
}

impl TryFrom<&str> for Interval {
    type Error = Error;

    fn try_from(string: &str) -> Result<Self> {
        match string {
            "min" | "m" => Ok(Interval::Minute),
            "hour" | "h" => Ok(Interval::Hour),
            "day" | "d" => Ok(Interval::Day),
            "week" | "w" => Ok(Interval::Week),
            _ => Err(ErrorKind::InvalidTimeInterval.into()),
        }
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
            AppSettings::GlobalVersion,
        ])
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
                        // IMPROVE: smth Interval::Week would be cleaner but &str is required by clap
                        .default_value("week"),
                )
                .arg(
                    Arg::with_name("precise").short("p").help(
                        "Precisely print timestamps in RFC 3339 format (includes milliseconds)",
                    ),
                )
                .arg(
                    Arg::with_name("rounding")
                        .short("r")
                        .takes_value(true)
                        .help("Rounding string to specify rounding options for time durations"),
                ),
        )
        .get_matches()
}
