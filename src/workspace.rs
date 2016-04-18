use std::fs;
use std::path;
use std::env;
use config::Config;
use errors::{Error, ErrorKind, Result};

/// Change directory to the workspace root.
///
/// # Errors
///
/// If changing the directory to the workspace root fails, return an `Error` of
/// kind `NotInWorkspace`.
pub fn cd_workspace_root(config: &Config) -> Result<()> {
    let workspace_root = try!(find_workspace_root(config));
    match env::set_current_dir(workspace_root) {
        Ok(_) => Ok(()),
        Err(_) => {
            Err(Error {
                kind: ErrorKind::IO,
                message: "Can not change directory to worspace root".to_string(),
            })
        }
    }
}

/// Find and return the path to the workspace root.
///
/// # Errors
///
/// If the current dir is not inside a workspace, return an `Error` of kind
/// `NotInWorkspace`.
pub fn find_workspace_root(config: &Config) -> Result<String> {
    // Get the current directory
    let mut current_dir = try!(current_dir());
    // If we are in a workspace return its path
    while !is_workspace_root(config, &current_dir) {
        let parent_dir = &String::from(current_dir);
        let parent_path = path::Path::new(parent_dir).parent();
        match parent_path {
            Some(path) => current_dir = try!(pathbuf_to_str(&path.to_path_buf())),
            None => {
                return Err(Error {
                    kind: ErrorKind::NotInWorkspace,
                    message: "Not in a workspace".to_string(),
                })
            }
        };
    }
    Ok(current_dir)
}

/// Return the path to the current directory as a `String`.
///
/// # Errors
///
/// If the current directory can not be retrived, return an `Error` of kind
/// `IO`.
fn current_dir() -> Result<String> {
    match env::current_dir() {
        Ok(pathbuf) => try!(Ok(pathbuf_to_str(&pathbuf))),
        Err(_) => {
            Err(Error {
                kind: ErrorKind::IO,
                message: "Can not get the currend directory path".to_string(),
            })
        }
    }
}

/// Cast a `PathBuf` to a `String`.
///
/// This is an helper function to avoid having to handle the `Option` result all
/// the time. Instead this return an error that can be returned directly.
///
/// # Errors
///
/// If casting the `PathBuf` to a `String` give no result, return an `Error` of
/// kind `IO`.
fn pathbuf_to_str(path: &path::PathBuf) -> Result<String> {
    match path.to_str() {
        Some(pathstr) => Ok(pathstr.to_string()),
        None => {
            Err(Error {
                kind: ErrorKind::IO,
                message: "One of the files in the hierarchy is not a valid UTF-8 string"
                             .to_string(),
            })
        }
    }
}

/// Check if the given path is the root of a workspace.
fn is_workspace_root(config: &Config, path: &str) -> bool {
    let read_dir = fs::read_dir(path);
    match read_dir {
        Ok(entries) => {
            for entry in entries {
                let filename = entry.unwrap().file_name();
                let fnstring = filename.to_str().unwrap();
                if fnstring == config.workspace_file {
                    return true;
                }
            }
            false
        }
        Err(_) => false,
    };
    false
}
