use crate::csv;
use crate::io;
use chrono::{Local};
use std::result::Result;

pub fn run() -> Result<(), &'static str> {
    println!("running subcommand IN");

    let file_path = io::build_path();

    let file = io::create_file_if_not_exists(&file_path).unwrap();
    let last_record = csv::read_last(&file_path);

    let now = Local::now();

    let record = match last_record {
        None => csv::build_first_record(now),
        Some(last) => {
            csv::validate_in(&last);
            csv::build_new_record(now, &last)
        },
    };

    println!("appending record: {:?}", record);

    csv::append_record(file, record).expect("Failed to write new record");

    Ok(())
}