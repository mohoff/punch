use std::fs;
use csv::{ReaderBuilder, WriterBuilder, StringRecord, Error};
use chrono::DateTime;
use chrono::offset::Local;

use serde::{Serialize, Deserialize};

use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    i: usize,
    start: String,
    end: Option<String>,
    note: Option<String>,
}


pub fn read_last(path: &PathBuf) -> Option<StringRecord> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)
        .unwrap();

    reader.records().last().map_or(None, |r| r.ok())
}

pub fn get_records(file_path: &PathBuf) -> std::iter::Peekable<impl Iterator<Item=Result<StringRecord, Error>>> {
    ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_path(&file_path)
        .expect("Could not read file")
        .into_records()
        .peekable()
}


pub fn build_first_record(timestamp: DateTime<Local>) -> StringRecord {
    StringRecord::from(vec![0.to_string(), timestamp.to_string()])
}

pub fn build_new_record(timestamp: DateTime<Local>, last_record: &StringRecord) -> StringRecord {
    let index = last_record.get(0)
        .expect("Could not get row index from row")
        .parse::<usize>()
        .expect("Could not parse row index to integer")
        .checked_add(1);

    match index {
        Some(i) => StringRecord::from(vec![i.to_string(), timestamp.to_string()]),
        None => panic!("Auto-increment of record index caused integer overflow."),
    }
}

pub fn build_terminated_record(timestamp: DateTime<Local>, last_record: &StringRecord) -> StringRecord {
    let mut new = last_record.clone();
    new.push_field(&timestamp.to_string());

    new
}

pub fn validate_in(record: &StringRecord) {
    if record.len() != 3 {
        panic!("Cannot start new entry without last record being terminated")
    }
}

pub fn validate_out(record: &StringRecord) {
    if record.len() != 2 {
        panic!("Cannot end entry as last record was terminated already")
    }
}

pub fn append_record(file: fs::File, record: StringRecord) -> std::result::Result<(), std::io::Error> {
    let mut writer = WriterBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_writer(file);

    writer.write_record(record.iter())?;
    writer.flush()
}