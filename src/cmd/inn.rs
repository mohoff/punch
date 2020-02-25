use crate::csv;
use crate::io;
use crate::err::*;
use chrono::{Local};
use colored::*;

pub fn run() -> Result<()> {
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

    csv::append_record(file, record).chain_err(|| "Unable to append record to file")?;

    print_success(now.to_rfc3339());

    Ok(())
}

fn print_success(time: String) {
    let suffix = format!("at {}", time);
    println!("👊 in - {}", suffix.dimmed());
}