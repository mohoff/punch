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
        FileDoesNotExist(path: String) {
            display("File does not exist: {}", path),
        }
        InvalidFile(path: String) {
            display("Invalid file: {}", path),
        }
        FileIsEmpty {
            display("File is empty"),
        }
        IncorrectCardStateForIn {
            display("Cannot punch in. Did you punch out last time?"),
        }
        IncorrectCardStateForOut {
            display("Cannot punch out. Did you punch in before?"),
        }
        InvalidTimeInterval {
            display("Failed to parse time interval"),
        }
        InvalidRoundingDirection {
            display("Failed to parse rounding direction"),
        }
    }
}

pub fn handle_error(error: &Error) {
    eprintln!("{}: {}", Red.paint("[punch error]"), error);
}
