//! This modules handle errors in prevy.
//!
//! The display of errorr doesn't use

// ------------------------------------------------------------------------- //
// Imports                                                                   //
// ------------------------------------------------------------------------- //

// Standard libraries imports
use std::process;
use std::result::Result as StdResult;

// External crates imports
use ansi_term::Colour::Red;

// Project imports
use core::utils::stderr;

// ------------------------------------------------------------------------- //
// Types                                                                     //
// ------------------------------------------------------------------------- //

pub type Result<T> = StdResult<T, Error>;

// ------------------------------------------------------------------------- //
// Structures                                                                //
// ------------------------------------------------------------------------- //

/// The different kinds of errors.
#[allow(dead_code)]
pub enum ErrorKind {
    /// Occurs in case of IO related error.
    IO,
    /// Occurs when the program is run outside of a workspace.
    NotInWorkspace,
    /// Occurs in case of parsing error.
    Parse,
}

/// A structure
pub struct Error {
    /// The kind of error.
    pub kind: ErrorKind,
    /// The error message.
    pub message: String,
    /// An optional error content.
    pub error: Option<String>,
}

impl Error {
    /// Exit the program by displaying an error message and returning 1.
    pub fn exit(&self) -> ! {
        let mut msg = self.message.clone();
        match self.error {
            None => (),
            Some(_) => msg.push(':'),
        }
        stderr(&Red.paint(msg).to_string());
        match self.error.clone() {
            None => (),
            Some(error) => stderr(&indent(error)),
        }
        process::exit(1);
    }
}

// ------------------------------------------------------------------------- //
// Traits                                                                    //
// ------------------------------------------------------------------------- //

/// A fallible unwrap
pub trait Fallible<T> {
    /// Return the value from Ok Result, or fail in case of Error.
    fn unwrap_or_fail(self) -> T;
}

/// Implements Fallible for Result
impl<T> Fallible<T> for Result<T> {
    fn unwrap_or_fail(self) -> T {
        match self {
            Ok(val) => val,
            Err(err) => err.exit(),
        }
    }
}

// ------------------------------------------------------------------------- //
// Internal functions                                                        //
// ------------------------------------------------------------------------- //

/// Indent a text with a tabulation.
///
/// Takes care of indenting all lines, not only the first one.
fn indent(text: String) -> String {
    let prefix = Red.paint(" │ ").to_string();
    prefix.clone() + &text.replace("\n", &("\n".to_string() + &prefix))
}
