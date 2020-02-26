use ansi_term::Colour::Red;

error_chain! {
    foreign_links {
        Clap(::clap::Error);
        Io(::std::io::Error);
        Csv(::csv::Error);
    }

    errors {
        HomeDirNotFound {
            display("Failed to determine home directory."),
        }
        FileDoesNotExist(file_path: String) {
            display("File does not exist: {}", file_path),
        }
        FileIsEmpty {
            display("File is empty"),
        }
        LastRecordHasIncorrectStateForIn {
            display("Cannot punch in. Did you punch out last time?"),
        }
        LastRecordHasIncorrectStateForOut {
            display("Cannot punch out. Did you punch in before?"),
        }
    }
}

pub fn handle_error(error: &Error) {
    eprintln!("{}: {}", Red.paint("[punch error]"), error);
}