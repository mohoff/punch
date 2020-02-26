use crate::csv;
use crate::record::{Record, RecordBucket};
use crate::cli::Interval;
use std::collections::BTreeMap;
use crate::io;
use crate::err::*;


pub fn run(interval: Interval) -> Result<()> {
    let file_path = io::build_path()?;
    io::validate_file_exists(&file_path)?;

    let mut reader = csv::build_reader(&file_path)?;

    let bucket_map = reader.deserialize()
        .map(Record::unwrap)
        .fold(BTreeMap::new(), |mut acc, record| {
            let key = record.bucket_key(interval);
            acc.entry(key)
                .or_insert(RecordBucket::new(interval))
                .add(record);

            acc
        });

    for bucket in bucket_map.values() {
        println!("{}", bucket);
    }

    Ok(())
}