// ------------------------------------------------------------------------- //
// Imports                                                                   //
// ------------------------------------------------------------------------- //

// Standard libraries imports
use std::fs;
use std::path;
use std::env;

// Project imports
use context::Context;
use core::errors::{Error, ErrorKind, Fallible, Result};
use core::utils::current_dir;
use vcs::Repo;

// ------------------------------------------------------------------------- //
// Structures                                                                //
// ------------------------------------------------------------------------- //

#[derive(Clone, Debug)]
pub struct Workspace {
    pub root: String,
    pub repos: Vec<Repo>,
}

impl Workspace {
    pub fn cd_root(&self) {
        cd_workspace_root(&self);
    }
}

impl Default for Workspace {
    fn default() -> Workspace {
        Workspace {
            root: "".to_string(),
            repos: Vec::default(),
        }
    }
}

// ------------------------------------------------------------------------- //
// Public API                                                                //
// ------------------------------------------------------------------------- //

pub fn parse_workspace(ctx: &mut Context) {
    match ctx.workspace_file_content["repos"].as_hash() {
        None => (),
        Some(hashmap) => {
            for repo in hashmap {
                ctx.workspace.repos.push(Repo::from_hash(repo).unwrap_or_fail())
            }
        }
    }
}

/// Change directory to the workspace root.
///
/// # Errors
///
/// If changing the directory to the workspace root fails, return an `Error` of
/// kind `IO`.
pub fn cd_workspace_root(ws: &Workspace) {
    match env::set_current_dir(ws.root.clone()) {
        Ok(_) => (),
        Err(error) => {
            Error {
                kind: ErrorKind::IO,
                message: "Can not change directory to worspace root".to_string(),
                error: Some(error.to_string()),
            }
            .exit()
        }
    };
}

/// Find and return the path to the workspace root.
///
/// # Errors
///
/// If the current dir is not inside a workspace, return an `Error` of kind
/// `NotInWorkspace`.
pub fn find_workspace_root(filename: String) -> Result<String> {
    // Get the current directory
    let mut current_dir = current_dir().unwrap_or_fail();
    // If we are in a workspace return its path
    while !is_workspace_root(&current_dir, filename.clone()) {
        let parent_dir = current_dir;
        let parent_path = path::Path::new(&parent_dir).parent();
        match parent_path {
            Some(path) => current_dir = path.to_str().unwrap().to_string(),
            None => {
                return Err(Error {
                    kind: ErrorKind::NotInWorkspace,
                    message: "Not in a workspace".to_string(),
                    error: None,
                })
            }
        };
    }
    Ok(current_dir)
}

// ------------------------------------------------------------------------- //
// Internal functions                                                        //
// ------------------------------------------------------------------------- //

/// Check if the given path is the root of a workspace.
fn is_workspace_root(path: &str, workspace_filename: String) -> bool {
    let read_dir = fs::read_dir(path);
    match read_dir {
        Ok(entries) => {
            for entry in entries {
                let filename = entry.unwrap().file_name();
                let fnstring = filename.to_str().unwrap();
                if fnstring == &workspace_filename {
                    return true;
                }
            }
            false
        }
        Err(_) => false,
    };
    false
}
