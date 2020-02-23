use std::fs;
use std::io;
use std::path::{PathBuf, Path};
use dirs;
use csv::Writer;

const TT_FILE: &'static str = "main.csv";
const TT_HOME_DIR_PATH: &'static str = ".timetracker";

pub fn build_path() -> PathBuf {
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => panic!("Could not find home dir"),
    };

    println!("homedir: {:?}", home_dir);

    let dir = home_dir.join(TT_HOME_DIR_PATH);
    println!("dir: {:?}", dir);
    fs::create_dir_all(&dir).expect("Could not create directory");

    let file_path = dir.join(TT_FILE);
    println!("file_path: {:?}", file_path);

    file_path
}

pub fn create_file_if_not_exists(file_path: &PathBuf) -> io::Result<fs::File> {
    fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)
}

pub fn validate_file_exists(file_path: &PathBuf) {
    if !Path::new(file_path).is_file() {
        panic!("Cannot end entry as file does not exist");
    }
}

pub fn flush_to_file(writer: Writer<std::vec::Vec<u8>>, file_path: &PathBuf) {
    let data = writer.into_inner().unwrap();
    fs::write(&file_path, data).expect("Unable to write file");
}