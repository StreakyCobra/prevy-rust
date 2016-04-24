// ------------------------------------------------------------------------- //
// Imports                                                                   //
// ------------------------------------------------------------------------- //

// Standard libraries imports
use std::collections::HashMap;
use std::collections::BTreeMap;

// External crates imports
use yaml_rust::Yaml;

// Project imports
use core::errors::{Error, ErrorKind};

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
    pub fn from_hash(hash: (&Yaml, &Yaml)) -> Repo {
        let mut repo = Repo {
            path: as_string(hash.0),
            kind: RepoKind::Git,
            remotes: HashMap::default(),
        };
        for (remote, url) in hash.1["remotes"].as_hash().unwrap_or(&BTreeMap::default()) {
            repo.remotes.insert(as_string(remote), as_string(url));
        };
        repo
    }
}

// ------------------------------------------------------------------------- //
// Internal functions                                                        //
// ------------------------------------------------------------------------- //

fn as_string(yaml: &Yaml) -> String {
    match yaml.as_str() {
        None => Error {
            kind: ErrorKind::Parse,
            message: "Error while parsing a value as string".to_string(),
            error: Some(format!("{:#?}", yaml)),
        }.exit(),
        Some(val) => val.to_string(),
    }
}
