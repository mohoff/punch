use std::fs::{self, File};
use std::path::PathBuf;
use std::collections::BTreeMap;

use ::csv::{Reader, ReaderBuilder, WriterBuilder, Writer};
use chrono::{DateTime, Local};
use dirs;
use colored::*;

use crate::err::*;
use crate::cli::Interval;
use crate::record::{Record, RecordBucket};
use crate::format::{Formatter, FormatRecordOptions};

const CARD_EXT: &'static str = "csv";
const CARD_NAME_DEFAULT: &'static str = "main";
const CARD_DIR: &'static str = ".punch";

pub struct Card(PathBuf);

impl Card {
    fn new(path: PathBuf) -> Result<Self> {
        Ok(Card(path))
    }

    #[allow(dead_code)]
    fn name(&self) -> &str {
        self.0.file_stem()
            .expect("Could not get card name")
            .to_str()
            .expect("Could not convert card path to name")
    }

    pub fn punch_in(&self, timestamp: DateTime<Local>, note: Option<&str>) -> Result<()> {
        let mut reader = self.get_reader()?;
        
        let mut records = reader.deserialize()
            .filter_map(std::result::Result::ok)
            .collect::<Vec<Record>>();

        // Check if all existing records have an end date
        if records.iter().all(|r| r.is_terminated()) == false {
            return Err(ErrorKind::IncorrectCardStateForIn.into());
        }

        records.push(Record::from((
            timestamp,
            records.len(),
            note.map(String::from),
        )));

        let writer = self.get_writer()?;
        Card::write_records_to_file(writer, records)
    }

    pub fn punch_out(&self, timestamp: DateTime<Local>, note: Option<String>) -> Result<()> {
        let mut reader = self.get_reader()?;

        let mut records = reader.deserialize()
            .filter_map(std::result::Result::ok)
            .collect::<Vec<Record>>();

        let last = records.pop();

        // Check that all 0..n-1 records have an end
        // date and that record n can be terminated.
        if records.iter().all(|r| r.is_terminated()) == false
            || last.is_none()
            || last.as_ref().unwrap().end.is_some()
        {
            return Err(ErrorKind::IncorrectCardStateForOut.into());
        }

        let mut last = last.unwrap();
        last.end.replace(timestamp);

        if let Some(snd) = note {
            let new_note = last.note
                .as_ref()
                .map_or(snd.clone(), |fst| format!("{};{}", fst, snd));
            last.note.replace(new_note);
        }
        records.push(last);

        let writer = self.get_writer()?;
        Card::write_records_to_file(writer, records)
    }

    pub fn display_with(&self, interval: Interval, precise: bool) -> Result<()> {
        let mut reader = self.get_reader()?;

        let mut num_total_records = 0;
        let bucket_map = reader.deserialize()
            .filter_map(std::result::Result::ok)
            .fold(BTreeMap::new(), |mut acc, record: Record| {
                num_total_records += 1;
        
                let key = record.bucket_key(interval);
                acc.entry(key)
                    .or_insert(RecordBucket::new(interval, precise))
                    .add(record);

                acc
            });

        let opts = FormatRecordOptions {
            align_with_n_records: num_total_records,
            precise,
        };

        println!("Showing card {}\n", self.name().bold());
        for bucket in bucket_map.values() {
            println!("{}", Formatter::format_bucket(bucket, &opts));
        }

        Ok(())
    }

    fn get_reader(&self) -> Result<Reader<std::fs::File>> {
        ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_path(&self.0)
            .chain_err(|| "Could not initialize reader")
    }

    fn get_writer(&self) -> Result<Writer<std::fs::File>> {
        WriterBuilder::new()
            .flexible(true)
            .has_headers(false)
            .from_path(&self.0)
            .chain_err(|| "Could not initialize writer")
    }

    fn write_records_to_file(mut writer: Writer<File>, records: Vec<Record>) -> Result<()> {
        for r in records {
            writer.serialize(r)?;
        }

        writer.flush().chain_err(|| "Could not write to card file")
    }
}

impl Default for Card {
    fn default() -> Self {
        let home_dir = dirs::home_dir().expect("Failed to get home dir");

        let dir = home_dir.join(CARD_DIR);

        fs::create_dir_all(&dir).expect("Could not create directory to store punch cards");

        let mut card_path = dir.join(CARD_NAME_DEFAULT);
        card_path.set_extension(CARD_EXT);

        Card::new(card_path).expect("Failed to create default punch card")
    }
}