extern crate ansi_term;
extern crate clap;
extern crate xdg_basedir;
extern crate yaml_rust;

mod config;
mod context;
mod display;
mod errors;
mod workspace;

use clap::{App, Arg, ArgMatches};

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
        .arg(Arg::with_name("workspace_filename")
                 .short("f")
                 .long("filename")
                 .help("Workspace filename")
                 .value_name("FILE")
                 .takes_value(true))
        .arg(Arg::with_name("debug")
                 .short("d")
                 .help("Enable debug output"))
        .get_matches()
}

/// Run prevy.
fn main() {
    // Parse command line arguments
    let args = parse_arguments();

    // Build the context
    let ctx = context::build_context(args);


    display::debug(&format!("{:#?}", ctx));
}
