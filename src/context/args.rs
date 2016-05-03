// ------------------------------------------------------------------------- //
// Imports                                                                   //
// ------------------------------------------------------------------------- //

// Standard libraries imports
use std::env;

// External crates imports
use atty;
use clap::{App, Arg, ArgMatches, AppSettings};

// Project imports
use core::constants::*;

// ------------------------------------------------------------------------- //
// Public API                                                                //
// ------------------------------------------------------------------------- //

/// Parse command line arguments.
pub fn parse_arguments() -> ArgMatches<'static> {
    let mut args = App::new("prevy")
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
                       .arg(Arg::with_name(ID_CONFIG_DEBUG)
                                .short("d")
                                .help("Enable debug output")
                                .hidden(true))
                       .arg(Arg::with_name(ID_CONFIG_NOCOLOR)
                                .long("no-color")
                                .help("Disable colored output"));

    // Chiken and egg problem here. We have to parse the command line
    // arguments to know if we should enable color, and we should decide if we
    // enable color for the arguments parser. The solution is to manually look
    // inside arguments for the "no-color" option

    // If the output is a TTY and there is not "no-color" somewhere in the
    // arguments, enable the colors.
    let nocolor_arg = "--".to_string() + ID_CONFIG_NOCOLOR;
    args = match atty::is() && !env::args().any(|s| s == nocolor_arg) {
        false => args,
        true => args.setting(AppSettings::ColoredHelp),
    };

    // Do the parsing and return the parsed arguments
    args.get_matches()
}
