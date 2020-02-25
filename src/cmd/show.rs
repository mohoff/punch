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
        // IMPROVE: group_by_interval and fold do similar things -> this can be improved
        .map(Record::group_by_interval(interval))
        .fold(BTreeMap::new(), |mut acc, (key, record)| {
            let bucket = acc.entry(key).or_insert(RecordBucket::new(interval));
            bucket.add(record);

            acc
        });

    for bucket in bucket_map.values() {
        println!("{}", bucket);
    }

    Ok(())
}