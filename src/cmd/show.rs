use crate::csv::{self, Record};
use crate::io;
use crate::err::*;


pub fn run() -> Result<()> {
    println!("running subcommand SHOW");

    let file_path = io::build_path()?;
    io::validate_file_exists(&file_path)?;

    let mut reader = csv::build_reader(&file_path)?;

    for record in reader.deserialize() {
        let record: Record = record?;
        println!("{}", record);
    }

    Ok(())
}