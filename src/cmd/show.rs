use crate::csv;
use crate::record::{Record, RecordBucket};
use crate::cli::Interval;
use std::collections::BTreeMap;
use chrono::Datelike;
use crate::io;
use crate::err::*;


pub fn run(interval: Interval) -> Result<()> {
    println!("running subcommand SHOW");

    let file_path = io::build_path()?;
    io::validate_file_exists(&file_path)?;

    let mut reader = csv::build_reader(&file_path)?;

    // for record in reader.deserialize() {
    //     let record: Record = record?;
    //     println!("{}", record);
    // }

    let grouped_records = reader.deserialize()
        // IMPROVE: idiomatic way?
        .map(|r| {
            let record: Record = r.unwrap();
            record
        })
        .map(|r| {
            let key = match interval {
                Interval::Day => r.start.day(),
                Interval::Week => r.start.iso_week().week(),
                Interval::Month => r.start.month(),
                Interval::Year => r.start.year() as u32,
            };

            (key, r)
        })
        .fold(BTreeMap::new(), |mut acc, (key, record)| {
            let bucket = acc.entry(key).or_insert(RecordBucket::new());
            bucket.add(record);

            acc
        });

    grouped_records.values().for_each(|bucket| {
        println!("{}", bucket.name_formatted(interval));
        println!("{}", bucket.stats_formatted());
        for record in bucket.records_formatted() {
            println!("{}", record);
        }
        println!("");
    });

    Ok(())
}