use std::io::{self, Write};
use ansi_term::Colour::{Red, Blue};

/// Print a given text to the standard output.
fn stdout(text: &str) {
    writeln!(io::stdout(), "{}", text).expect("Error writing Error to stdout");
}

/// Print a given text to the standard error.
fn stderr(text: &str) {
    writeln!(io::stderr(), "{}", text).expect("Error writing Error to stderr");
}

/// Print an error message.
pub fn error(text: &str) {
    stderr(&Red.paint(text).to_string());
}

/// Print an info message.
pub fn info(text: &str) {
    stdout(&Blue.paint(text).to_string());
}
