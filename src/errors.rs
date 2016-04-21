use display;

use std::process;
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

/// The different kinds of errors.
pub enum ErrorKind {
    /// Occurs in case of IO related error.
    IO,
    /// Occurs when the program is run outside of a workspace.
    NotInWorkspace,
    /// Occurs in case of parsing error.
    Parser,
}

/// Handling errors
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
        display::error(&self.message);
        match self.error.clone() {
            None => (),
            Some(error) => display::text(&error),
        }
        process::exit(1);
    }
}
