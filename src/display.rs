use std::io::{self, Write};
use ansi_term::Colour::{Red, Blue};

/// Print an error message.
pub fn error(text: &str) {
    writeln!(io::stderr(), "{}", Red.paint(text)).expect("Error writing Error to stderr");
}

/// Print an info message.
pub fn info(text: &str) {
    writeln!(io::stdout(), "{}", Blue.paint(text)).expect("Error writing Error to stdout");
}
