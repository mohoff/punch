use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;
use std::fmt;

use ::csv::{Reader, ReaderBuilder, Writer, WriterBuilder};
use colored::*;
use dirs;

use crate::bucket::RecordBucket;
use crate::err::*;
use crate::format::CardFormattingOptions;
use crate::record::Record;
use crate::time::Timestamp;

const CARD_EXT: &str = "csv";
const CARD_NAME_DEFAULT: &str = "main";
const CARD_DIR: &str = ".punch";

pub struct Card(PathBuf);

#[derive(Debug)]
pub enum CardStatus {
    PunchedIn,
    PunchedOut,
    Corrupted,
}

impl fmt::Display for CardStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Card {
    fn new(path: PathBuf) -> Result<Self> {
        // Assumes that the directory already exists
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path)
            .chain_err(|| "Failed to create card file")?;

        Ok(Card(path))
    }

    #[allow(dead_code)]
    fn name(&self) -> &str {
        self.0
            .file_stem()
            .expect("Could not get card name")
            .to_str()
            .expect("Could not convert card path to name")
    }
    pub fn path(&self) -> &PathBuf {
        &self.0
    }

    pub fn status(&self) -> Result<CardStatus> {
        let mut reader = self.get_reader()?;

        let mut records = reader
            .deserialize()
            .filter_map(std::result::Result::ok)
            .collect::<Vec<Record>>();

        let last = records.pop();

        if !records.iter().all(|r| r.is_terminated()) {
            Ok(CardStatus::Corrupted)
        } else if last.is_none() || last.as_ref().unwrap().end.is_some() {
            Ok(CardStatus::PunchedOut)
        } else {
            Ok(CardStatus::PunchedIn)
        }
    }

    pub fn punch_in(&self, timestamp: Timestamp, note: Option<&str>) -> Result<()> {
        let mut reader = self.get_reader()?;

        let mut records = reader
            .deserialize()
            .filter_map(std::result::Result::ok)
            .collect::<Vec<Record>>();

        // Check if all existing records have an end date
        if !records.iter().all(|r| r.is_terminated()) {
            return Err(ErrorKind::IncorrectCardStateForIn.into());
        }

        records.insert(0, Record::from((
            timestamp,
            records.len(),
            note.map(String::from),
        )));

        let writer = self.get_writer()?;
        Card::write_records_to_file(writer, records)
    }

    pub fn punch_out(&self, timestamp: Timestamp, note: Option<String>) -> Result<()> {
        let mut reader = self.get_reader()?;

        let mut records = reader
            .deserialize()
            .filter_map(std::result::Result::ok)
            .collect::<Vec<Record>>();

        // Check that all 1..n records have an end date
        // and that the first record can be terminated.
        if !records.iter().skip(1).all(|r| r.is_terminated())
            || records.first().is_none()
            || records.first().as_ref().unwrap().end.is_some()
        {
            return Err(ErrorKind::IncorrectCardStateForOut.into());
        }

        let first = records.first_mut().unwrap();
        first.end.replace(timestamp);

        if let Some(snd) = note {
            let new_note = first
                .note
                .as_ref()
                .map_or(snd.clone(), |fst| format!("{};{}", fst, snd));
            first.note.replace(new_note);
        }

        let writer = self.get_writer()?;
        Card::write_records_to_file(writer, records)
    }

    pub fn display_with(&self, mut opts: CardFormattingOptions) -> Result<()> {
        let mut reader = self.get_reader()?;

        let mut num_total_records = 0;
        let bucket_map = reader
            .deserialize()
            .filter_map(std::result::Result::ok)
            .fold(BTreeMap::new(), |mut acc, record: Record| {
                num_total_records += 1;
                let key = record.bucket_key(opts.interval);

                #[allow(clippy::or_fun_call)]
                acc.entry(key)
                    .or_insert(RecordBucket::new(opts.interval, opts.record_opts.precise))
                    .add(record);

                acc
            });

        println!("Showing card {}\n", self.name().bold());

        match num_total_records {
            0 => println!("{}\n", "no punches yet".italic().dimmed()),
            n => {
                opts.record_opts.align_with_n_records = n;

                for bucket in bucket_map.values() {
                    bucket.display_with(&opts.record_opts);
                }
            }
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
