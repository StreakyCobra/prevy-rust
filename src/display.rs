use std::io::{self, Write};
use ansi_term::Colour::{Red, Blue, Yellow, Purple};

/// Print a given text to the standard output.
fn stdout(text: &str) {
    writeln!(io::stdout(), "{}", text).expect("Error writing Error to stdout");
}

/// Print a given text to the standard error.
fn stderr(text: &str) {
    writeln!(io::stderr(), "{}", text).expect("Error writing Error to stderr");
}

/// Print an error message.
pub fn print(text: &str) {
    stderr(text);
}

/// Print an error message.
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

/// Print an info message.
pub fn warn(text: &str) {
    stdout(&Yellow.paint(text).to_string());
}
