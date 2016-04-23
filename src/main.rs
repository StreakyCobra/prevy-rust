// External crates
extern crate ansi_term;
extern crate clap;
extern crate xdg_basedir;
extern crate yaml_rust;

// Project modules
mod config;
mod constants;
mod context;
mod display;
mod errors;
mod utils;
mod workspace;

// External crates imports
use clap::{App, Arg, ArgMatches};

// Project imports
use constants::*;

/// Parse command line arguments.
fn parse_arguments<'a>() -> ArgMatches<'a> {
    App::new("prevy")
        .version("0.1.0")
        .author("Fabien Dubosson <fabien.dubosson@gmail.com>")
        .about("Manage your development workspaces with ease.")
        .arg(Arg::with_name(ID_CONFIGURATION_FILE)
                 .short("c")
                 .long("config")
                 .help("Path to the configuration file")
                 .value_name("FILE")
                 .takes_value(true))
        .arg(Arg::with_name(ID_WORKSPACE_FILENAME)
                 .short("f")
                 .long("filename")
                 .help("Name of the workspace file")
                 .value_name("FILENAME")
                 .takes_value(true))
        .arg(Arg::with_name("debug")
                 .short("d")
                 .help("Enable debug output")
                 .hidden(true))
        .get_matches()
}

/// Run prevy.
fn main() {
    // Parse command line arguments
    let args = parse_arguments();

    // Build the context
    let ctx = context::build_context(args);

    // Print the context if debug is enabled
    if ctx.config.debug {
        display::debug(&format!("{:#?}", ctx));
    }
}
