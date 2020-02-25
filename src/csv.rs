use std::fs;
use std::fmt;
use csv::{Reader, ReaderBuilder, WriterBuilder, StringRecord, Error};
use chrono::{DateTime, Duration};
use chrono::offset::{Local};
use colored::*;

use std::path::PathBuf;

use crate::err::*;

pub fn build_reader(file_path: &PathBuf) -> Result<Reader<std::fs::File>> {
    ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path(file_path)
        .chain_err(|| "Could not initialize reader")
}

pub fn read_last(file_path: &PathBuf) -> Result<StringRecord> {
    let mut reader = build_reader(file_path)?;

    match reader.records().last() {
        None => Err(ErrorKind::FileIsEmpty.into()),
        Some(last) => last.chain_err(|| "Error while reading last record"),
    }
}

pub fn get_records(file_path: &PathBuf) -> Result<std::iter::Peekable<impl Iterator<Item=std::result::Result<StringRecord, Error>>>> {
    let reader = build_reader(file_path)?;

    Ok(reader.into_records().peekable())
}


pub fn build_first_record(timestamp: DateTime<Local>) -> StringRecord {
    StringRecord::from(vec!["0", &timestamp.to_rfc3339()])
}

pub fn build_new_record(timestamp: DateTime<Local>, last_record: &StringRecord) -> StringRecord {
    let index = last_record.get(0)
        .expect("Could not get row index from row")
        .parse::<usize>()
        .expect("Could not parse row index to integer")
        .checked_add(1);

    match index {
        Some(i) => StringRecord::from(vec![i.to_string(), timestamp.to_rfc3339()]),
        None => panic!("Auto-increment of record index caused integer overflow."),
    }
}

pub fn build_terminated_record(timestamp: DateTime<Local>, last_record: &StringRecord) -> StringRecord {
    let mut new = last_record.clone();
    new.push_field(&timestamp.to_rfc3339());

    new
}

pub fn validate_in(record: &StringRecord) -> Result<()> {
    if record.len() != 3 {
        Err(ErrorKind::LastRecordHasIncorrectStateForIn.into())
    } else {
        Ok(())
    }
}

pub fn validate_out(record: &StringRecord) -> Result<()> {
    if record.len() != 2 {
        Err(ErrorKind::LastRecordHasIncorrectStateForOut.into())
    } else {
        Ok(())
    }
}

pub fn append_record(file: fs::File, record: StringRecord) -> Result<()> {
    let mut writer = WriterBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_writer(file);

    writer.write_record(record.iter())?;
    writer.flush()?;

    Ok(())
}