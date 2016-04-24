// ------------------------------------------------------------------------- //
// Imports                                                                   //
// ------------------------------------------------------------------------- //

// Standard libraries imports
use std::collections::HashMap;
use std::collections::BTreeMap;

// External crates imports
use yaml_rust::Yaml;

// Project imports
use core::errors::{Error, ErrorKind, Result};

// ------------------------------------------------------------------------- //
// Structures                                                                //
// ------------------------------------------------------------------------- //

#[derive(Clone, Debug)]
pub enum RepoKind {
    Git,
}

#[derive(Clone, Debug)]
pub struct Repo {
    pub path: String,
    pub kind: RepoKind,
    pub remotes: HashMap<String, String>,
}

impl Repo {
    pub fn from_hash(hash: (&Yaml, &Yaml)) -> Result<Repo> {
        let mut repo = Repo {
            path: try!(as_string(hash.0)),
            kind: RepoKind::Git,
            remotes: HashMap::default(),
        };
        for (remote, url) in hash.1["remotes"].as_hash().unwrap_or(&BTreeMap::default()) {
            repo.remotes.insert(try!(as_string(remote)), try!(as_string(url)));
        }
        Ok(repo)
    }
}

// ------------------------------------------------------------------------- //
// Internal functions                                                        //
// ------------------------------------------------------------------------- //

fn as_string(yaml: &Yaml) -> Result<String> {
    match yaml.as_str() {
        None => {
            Err(Error {
                kind: ErrorKind::Parse,
                message: "Error while parsing a value as string".to_string(),
                error: Some(format!("{:#?}", yaml)),
            })
        }
        Some(val) => Ok(val.to_string()),
    }
}
