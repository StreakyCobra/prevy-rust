extern crate ansi_term;
extern crate clap;

mod config;
mod display;
mod errors;
mod workspace;

use config::Config;
use clap::{App, Arg, ArgMatches};
use std::env;
use errors::Result;

/// Parse command line arguments and return them.
fn parse_arguments<'a>() -> ArgMatches<'a> {
    App::new("prevy")
        .version("0.1.0")
        .author("Fabien Dubosson <fabien.dubosson@gmail.com>")
        .about("Manage your development workspaces with ease.")
        .arg(Arg::with_name("configuration_file")
                 .short("c")
                 .long("config")
                 .help("User configuration file")
                 .value_name("FILE")
                 .takes_value(true))
        .arg(Arg::with_name("workspace_file")
                 .short("f")
                 .long("filename")
                 .help("Workspace file name")
                 .value_name("FILE")
                 .takes_value(true))
        .get_matches()
}

/// Run prevy.
fn main() {
    // Parse command line arguments
    let args = parse_arguments();

    // Create a default configuration
    let conf = &mut Config { args: args, ..Default::default() };

    // Update the configuration
    config::get_config(conf);

    // Change to project root
    try(workspace::cd_workspace_root(conf));

    // Check that project root have been changed by printing it
    let cwd_path = env::current_dir().unwrap();
    let cwd: &str = cwd_path.to_str().unwrap();
    display::info(cwd);

    display::warn(&conf.workspace_file);
    conf.workspace_file = "changed";
    display::warn(&conf.workspace_file);
}

/// Check if a function returned an error and handle it.
fn try<A>(result: Result<A>) {
    match result {
        Ok(_) => (),
        Err(err) => err.exit(),
    }
}
