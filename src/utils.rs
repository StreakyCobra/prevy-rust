use std::fs::File;
use std::io::prelude::*;

use yaml_rust::{Yaml, YamlLoader};

use errors::{Error, ErrorKind, Result};

///
pub fn read_yaml_file(filename: String) -> Result<Yaml> {
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

    let mut content = String::new();

    let _ = file.read_to_string(&mut content);

    match YamlLoader::load_from_str(&content) {
        Err(error) => {
            Err(Error {
                kind: ErrorKind::Parse,
                message: format!{"Error while parsing '{}'", filename}.to_string(),
                error: Some(error.to_string()),
            })
        }
        Ok(yaml) => Ok(yaml[0].clone()),
    }
}
