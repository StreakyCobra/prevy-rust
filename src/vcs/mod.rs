pub mod git;

// ------------------------------------------------------------------------- //
// Imports                                                                   //
// ------------------------------------------------------------------------- //

// Standard libraries imports
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::fmt;

// External crates imports
use yaml_rust::Yaml;

// Project imports
use core::constants::*;
use core::errors::{Error, ErrorKind, Result};
use self::git::Git;

// ------------------------------------------------------------------------- //
// Structs                                                                   //
// ------------------------------------------------------------------------- //

#[derive(Clone, Debug)]
pub struct RepoInfo {
    pub path: String,
    pub url: String,
    pub remotes: HashMap<String, String>,
}

// ------------------------------------------------------------------------- //
// Traits                                                                    //
// ------------------------------------------------------------------------- //

pub trait RepoClone {
    fn clone_box(&self) -> Box<Repo>;
}

impl<T> RepoClone for T where T: 'static + Repo + Clone {
    fn clone_box(&self) -> Box<Repo> {
        Box::new(self.clone())
    }
}

pub trait Repo: RepoClone {
    fn from_info(RepoInfo) -> Self where Self: Sized;
    fn clone_repo(self);
}

impl fmt::Debug for Repo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hi: {}", "Test")
    }
}

impl Clone for Box<Repo> {
    fn clone(&self) -> Box<Repo> {
        self.clone_box()
    }
}

// ------------------------------------------------------------------------- //
// Public API                                                                //
// ------------------------------------------------------------------------- //

pub fn from_hash(hash: (&Yaml, &Yaml)) -> Result<Box<Repo>> {
    let mut repo_info = RepoInfo {
        path: try!(as_string(hash.0)),
        url: hash.1["url"].as_str().unwrap().to_string(),
        remotes: HashMap::default(),
    };
    for (remote, url) in hash.1[SEC_WORKSPACE_REMOTES].as_hash().unwrap_or(&BTreeMap::default()) {
        repo_info.remotes.insert(try!(as_string(remote)), try!(as_string(url)));
    }
    Ok(Box::new(Git::from_info(repo_info)))
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
