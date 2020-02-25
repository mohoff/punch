use std::fs;
use std::path::{PathBuf, Path};
use dirs;
use csv::Writer;

use crate::err::*;

const PUNCH_FILE: &'static str = "main.csv";
const PUNCH_HOME_DIR_PATH: &'static str = ".punch";

pub fn build_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().ok_or(ErrorKind::HomeDirNotFound)?;
    dbg!(&home_dir);

    let dir = home_dir.join(PUNCH_HOME_DIR_PATH);
    dbg!(&dir);

    fs::create_dir_all(&dir).expect("Could not create directory");

    let file_path = dir.join(PUNCH_FILE);
    dbg!(&file_path);

    Ok(file_path)
}

pub fn create_file_if_not_exists(file_path: &PathBuf) -> Result<fs::File> {
    fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)
            .chain_err(|| "Could not create or open file")
}

pub fn validate_file_exists(file_path: &PathBuf) -> Result<()> {
    match Path::new(file_path).is_file() {
        true => Ok(()),
        false => Err(ErrorKind::FileDoesNotExist(
            String::from(file_path.to_str().unwrap())
        ).into()),
    }
}

pub fn flush_to_file(writer: Writer<std::vec::Vec<u8>>, file_path: &PathBuf) -> Result<()> {
    let data = writer.into_inner().chain_err(|| "Could not flush writer")?;
    fs::write(&file_path, data).chain_err(|| "Could not write to file")
}