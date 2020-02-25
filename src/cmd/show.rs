use crate::csv;
use crate::record::{Record,RecordBucket,BucketType};
use std::collections::{BTreeMap,HashSet};
use chrono::Datelike;
use crate::io;
use crate::err::*;


pub fn run() -> Result<()> {
    println!("running subcommand SHOW");

    let file_path = io::build_path()?;
    io::validate_file_exists(&file_path)?;

    let mut reader = csv::build_reader(&file_path)?;

    // for record in reader.deserialize() {
    //     let record: Record = record?;
    //     println!("{}", record);
    // }

    // improve make this a cli param
    let group_by = BucketType::DAY;

    let grouped_records = reader.deserialize()
        // IMPROVE: idiomatic way?
        .map(|r| {
            let record: Record = r.unwrap();
            record
        })
        .map(|r| {
            let key = match group_by {
                BucketType::YEAR => r.start.year() as u32,
                BucketType::MONTH => r.start.month(),
                BucketType::DAY => r.start.day(),
                _ => r.start.day(),
            };

            (key, r)
        })
        .fold(BTreeMap::new(), |mut acc, (key, record)| {
            let bucket = acc.entry(key).or_insert(RecordBucket::new());
            bucket.add(record);

            acc
        });

    // TODO: implement struct RecordBucket(Vec<Record>) with methods .duration_sum/max/min/avg, .size
    // and .bucket_name(groupby: GroupBy)
    grouped_records.values().for_each(|bucket| {
        println!("{}", bucket.name_formatted(&group_by));
        println!("{}", bucket.stats_formatted());
        for record in bucket.records_formatted() {
            println!("{}", record);
        }
        println!("");
    });

    Ok(())
}