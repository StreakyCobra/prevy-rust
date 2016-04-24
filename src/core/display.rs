#![allow(dead_code)]

// ------------------------------------------------------------------------- //
// Imports                                                                   //
// ------------------------------------------------------------------------- //

// Standard libraries imports
use std::io::{self, Write};

// External crates imports
use ansi_term::Colour::{Red, Blue, Yellow, Purple};

// Project imports
use core::errors::{Error, ErrorKind};

// ------------------------------------------------------------------------- //
// Public API                                                                //
// ------------------------------------------------------------------------- //

/// Print a message.
pub fn print(text: &str) {
    stderr(text);
}

/// Print a debug message.
pub fn debug(text: &str) {
    stderr(&Blue.paint(text).to_string());
}

/// Print an error message.
pub fn error(text: &str) {
    stderr(&Red.paint(text).to_string());
}

/// Print an info message.
pub fn info(text: &str) {
    stdout(&Purple.paint(text).to_string());
}

/// Print a warning message.
pub fn warn(text: &str) {
    stdout(&Yellow.paint(text).to_string());
}

// ------------------------------------------------------------------------- //
// Internal functions                                                        //
// ------------------------------------------------------------------------- //

/// Print a text to the standard output.
fn stdout(text: &str) {
    match writeln!(io::stdout(), "{}", text) {
        Ok(_) => (),
        Err(err) => {
            Error {
                kind: ErrorKind::IO,
                message: "Error writing to standard output.".to_string(),
                error: Some(err.to_string()),
            }
            .exit()
        }
    }
}

/// Print a text to the standard error.
fn stderr(text: &str) {
    match writeln!(io::stderr(), "{}", text) {
        Ok(_) => (),
        Err(err) => {
            Error {
                kind: ErrorKind::IO,
                message: "Error writing to standard error.".to_string(),
                error: Some(err.to_string()),
            }
            .exit()
        }
    }
}
