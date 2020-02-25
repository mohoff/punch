use ::csv::Writer;
use crate::io;
use crate::csv;
use chrono::{Local};
use crate::err::*;
use colored::*;


pub fn run() -> Result<()> {
    let file_path = io::build_path()?;
    io::validate_file_exists(&file_path)?;

    let now = Local::now();

    let mut writer = Writer::from_writer(vec![]);
    let mut iter = csv::get_records(&file_path)?;
    
    while let Some(result) = iter.next() {
        let record = match iter.peek() {
            Some(_) => result.unwrap(),
            // when None is peeked, `result` represents the
            // last element which we want to modify
            None => {
                let last = result.unwrap();
                csv::validate_out(&last)?;
                csv::build_terminated_record(now, &last)
            },
        };

        writer.write_record(record.iter()).chain_err(|| "Could not write record to buffer")?;
    };

    io::flush_to_file(writer, &file_path).chain_err(|| "Could not flush data to file")?;

    print_success(now.to_rfc3339());

    Ok(())
}

fn print_success(time: String) {
    let suffix = format!("at {}", time);
    println!("👊 out - {}", suffix.dimmed());
}