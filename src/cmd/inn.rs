use crate::csv;
use crate::io;
use crate::err::*;
use chrono::{Local};
//use std::result::Result;
//use err::*;



pub fn run() -> Result<()> {
    println!("running subcommand IN");

    let file_path = io::build_path()?;

    let file = io::create_file_if_not_exists(&file_path).chain_err(|| "Could not create file")?;
    let last_record = csv::read_last(&file_path);

    let now = Local::now();

    let record = match last_record {
        Err(_) => csv::build_first_record(now),
        Ok(last) => {
            csv::validate_in(&last)?;
            csv::build_new_record(now, &last)
        },
    };

    println!("appending record: {:?}", record);

    csv::append_record(file, record).chain_err(|| "Unable to append record to file")?;

    Ok(())
}