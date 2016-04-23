// ------------------------------------------------------------------------- //
// Imports                                                                   //
// ------------------------------------------------------------------------- //

// Standard libraries imports
use std::fs::File;
use std::io::prelude::*;

// External crates imports
use yaml_rust::{Yaml, YamlLoader};

// Project imports
use core::errors::{Error, ErrorKind, Result};

// ------------------------------------------------------------------------- //
// Public API                                                                //
// ------------------------------------------------------------------------- //

/// Read a YAML file and return its content.
///
/// # Panics
///
/// This function will exit the current program if the file can not be parsed.
/// On the other side, this will not directly return if the file doesn't exists
/// as this is a choice that should be made by the caller. It will just return
/// the Result instead.
pub fn read_yaml_file(filename: String) -> Result<Yaml> {
    // Try to open the file
    let mut file = match File::open(filename.clone()) {
        Ok(file) => file,
        Err(error) => {
            return Err(Error {
                kind: ErrorKind::IO,
                message: format!{"Error when trying to open '{}'.", filename}.to_string(),
                error: Some(error.to_string()),
            })
        }
    };

    // Read the file content
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);

    // Parse the file content as YAML and return it
    match YamlLoader::load_from_str(&content) {
        Err(error) => {
            Error {
                kind: ErrorKind::Parse,
                message: format!{"Error while parsing '{}'", filename}.to_string(),
                error: Some(error.to_string()),
            }.exit();
        }
        Ok(yaml) => Ok(yaml[0].clone()),
    }
}
