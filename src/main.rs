extern crate ansi_term;
extern crate clap;
extern crate xdg_basedir;
extern crate yaml_rust;

mod config;
mod display;
mod errors;
mod workspace;

use clap::{App, Arg, ArgMatches};
use std::env;
use errors::Result;

/// Parse command line arguments.
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

    // Bootstrap the configuration
    // TODO

    // Parse the configuration
    let mut conf = config::parse_config(args);

    // Get workspace information
    // let workspace = workspace::get_workspace();

    // Move to project root
    workspace::cd_workspace_root(&conf);


    // Check that project root have been changed by printing it
    let cwd_path = env::current_dir().unwrap();
    let cwd: &str = cwd_path.to_str().unwrap();
    display::info(cwd);

    display::warn(&conf.workspace_file);
    conf.workspace_file = "changed".to_string();
    display::warn(&conf.workspace_file);
}
