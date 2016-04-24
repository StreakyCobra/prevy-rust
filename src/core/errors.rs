// ------------------------------------------------------------------------- //
// Imports                                                                   //
// ------------------------------------------------------------------------- //

// Standard libraries imports
use std::process;
use std::result::Result as StdResult;

// ------------------------------------------------------------------------- //
// Structures                                                                //
// ------------------------------------------------------------------------- //

pub type Result<T> = StdResult<T, Error>;

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
        print(&msg);
        match self.error.clone() {
            None => (),
            Some(error) => print(&indent(error)),
        }
        process::exit(1);
    }
}

pub trait Exitable<T> {
    fn handle_error(self) -> T;
}

impl<T> Exitable<T> for Result<T> {
    fn handle_error(self) -> T {
        match self {
            Ok(val) => val,
            Err(err) => err.exit(),
        }
    }
}

// ------------------------------------------------------------------------- //
// Internal functions                                                        //
// ------------------------------------------------------------------------- //

fn print(text: &str) {
    println!("{}", text);
}

fn indent(text: String) -> String {
    "\t".to_string() + &text.replace("\n", "\t")
}
