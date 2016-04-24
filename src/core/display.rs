#![allow(dead_code)]

// ------------------------------------------------------------------------- //
// Imports                                                                   //
// ------------------------------------------------------------------------- //

// Standard libraries imports
use std::io::{self, Write};
use std::process;

// External crates imports
use ansi_term::Colour::{self, Red, Blue, Yellow, Purple};

// Project imports
use context::Context;

// ------------------------------------------------------------------------- //
// Structure                                                                 //
// ------------------------------------------------------------------------- //

#[derive(Clone, Debug)]
pub struct Display {
    nocolor: bool,
}

impl Display {
    /// Print a message.
    pub fn print(self, text: &str) {
        stderr(text);
    }

    /// Print a debug message.
    pub fn debug(self, text: &str) {
        stderr(&Blue.paint(text).to_string());
    }

    /// Print an error message.
    pub fn error(self, text: &str) {
        stderr(&Red.paint(text).to_string());
    }

    /// Print an info message.
    pub fn info(self, text: &str) {
        stdout(&Purple.paint(text).to_string());
    }

    /// Print a warning message.
    pub fn warn(self, text: &str) {
        stdout(&Yellow.paint(text).to_string());
    }
}

impl Default for Display {
    fn default() -> Display {
        Display { nocolor: true }
    }
}

// ------------------------------------------------------------------------- //
// Public API                                                                //
// ------------------------------------------------------------------------- //

pub fn create_display(ctx: &mut Context) {
    ctx.display = Display { nocolor: ctx.config.nocolor }
}

/// Print a text to the standard output.
pub fn stdout(text: &str) {
    match writeln!(io::stdout(), "{}", text) {
        Ok(_) => (),
        // If it goes wrong, exit with a special code as we can't write anything.
        Err(_) => process::exit(1),
    }
}

/// Print a text to the standard error.
pub fn stderr(text: &str) {
    match writeln!(io::stderr(), "{}", text) {
        Ok(_) => (),
        // If it goes wrong, exit with a special code as we can't write anything.
        Err(_) => process::exit(1),
    }
}
/// Indent a text with a tabulation.
///
/// Takes care of indenting all lines, not only the first one.
pub fn boxify(text: String, color: Colour) -> String {
    let prefix = color.paint("│ ").to_string();
    let tail = color.paint("\n└ ").to_string();
    let mut result = String::new();
    result.push_str(&prefix);
    result.push_str(&text.replace("\n", &("\n".to_string() + &prefix)));
    result.push_str(&tail);
    result
}
