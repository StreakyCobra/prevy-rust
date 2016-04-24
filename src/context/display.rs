#![allow(dead_code)]

// ------------------------------------------------------------------------- //
// Imports                                                                   //
// ------------------------------------------------------------------------- //

// External crates imports
use ansi_term::Colour::{Red, Blue, Yellow, Purple};

// Project imports
use context::Context;
use core::utils::{stderr, stdout};

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

pub fn prepare_display(ctx: &mut Context) {
    ctx.display = Display { nocolor: ctx.config.nocolor }
}
