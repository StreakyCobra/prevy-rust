extern crate ansi_term;
extern crate clap;

mod display;
mod errors;
mod workspace;

use clap::{ArgMatches, App};
use std::env;
use errors::Result;

/// Parse command line arguments and return them.
fn parse_arguments<'a>() -> ArgMatches<'a> {
    App::new("prevy")
        .version("0.1.0")
        .author("Fabien Dubosson <fabien.dubosson@gmail.com>")
        .about("Manage your development workspaces with ease.")
        .get_matches()
}

fn main() {
    // Parse the arguments
    let _ = parse_arguments();

    // Change to project root
    check(workspace::cd_workspace_root());

    // Check that project root have been changed by printing it
    let cwd_path = env::current_dir().unwrap();
    let cwd: &str = cwd_path.to_str().unwrap();
    display::info(cwd);
}

/// Check if a function returned an error and handle it.
fn check<A>(result: Result<A>) {
    match result {
        Ok(_) => (),
        Err(err) => err.exit(),
    }
}
