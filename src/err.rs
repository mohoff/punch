
error_chain! {
    foreign_links {
        Clap(::clap::Error);
        Io(::std::io::Error);
        Csv(::csv::Error);
        //SyntectError(::syntect::LoadingError);
        //ParseIntError(::std::num::ParseIntError);
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
    // match error {
    //     Error(ErrorKind::Io(ref io_error), _)
    //         if io_error.kind() == ::std::io::ErrorKind::BrokenPipe =>
    //     {
    //         ::std::process::exit(0);
    //     }
    //     _ => {
    //         use ansi_term::Colour::Red;
    //         eprintln!("{}: {}", Red.paint("[punch error]"), error);
    //     }
    // };
    use ansi_term::Colour::Red;
    eprintln!("{}: {}", Red.paint("[punch error]"), error);
}

// error_chain! {
//     // The type defined for this error. These are the conventional
//     // and recommended names, but they can be arbitrarily chosen.
//     //
//     // It is also possible to leave this section out entirely, or
//     // leave it empty, and these names will be used automatically.
//     types {
//         Error, ErrorKind, ResultExt, Result;
//     }

//     // Without the `Result` wrapper:
//     //
//     // types {
//     //     Error, ErrorKind, ResultExt;
//     // }

//     // Automatic conversions between this error chain and other
//     // error chains. In this case, it will e.g. generate an
//     // `ErrorKind` variant called `Another` which in turn contains
//     // the `other_error::ErrorKind`, with conversions from
//     // `other_error::Error`.
//     //
//     // Optionally, some attributes can be added to a variant.
//     //
//     // This section can be empty.
//     links {
//         Another(other_error::Error, other_error::ErrorKind) #[cfg(unix)];
//     }

//     // Automatic conversions between this error chain and other
//     // error types not defined by the `error_chain!`. These will be
//     // wrapped in a new error with, in the first case, the
//     // `ErrorKind::Fmt` variant. The description and cause will
//     // forward to the description and cause of the original error.
//     //
//     // Optionally, some attributes can be added to a variant.
//     //
//     // This section can be empty.
//     foreign_links {
//         Fmt(::std::fmt::Error);
//         Io(::std::io::Error) #[cfg(unix)];
//     }

//     // Define additional `ErrorKind` variants.  Define custom responses with the
//     // `description` and `display` calls.
//     errors {
//         InvalidToolchainName(t: String) {
//             description("invalid toolchain name")
//             display("invalid toolchain name: '{}'", t)
//         }

//         // You can also add commas after description/display.
//         // This may work better with some editor auto-indentation modes:
//         UnknownToolchainVersion(v: String) {
//             description("unknown toolchain version"), // note the ,
//             display("unknown toolchain version: '{}'", v), // trailing comma is allowed
//         }
//     }

//     // If this annotation is left off, a variant `Msg(s: String)` will be added, and `From`
//     // impls will be provided for `String` and `&str`
//     skip_msg_variant
// }