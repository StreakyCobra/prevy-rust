extern crate clap;

use clap::{ArgMatches, App};

fn parse_arguments<'a>() -> ArgMatches<'a> {
    let matches = App::new("prevy")
        .version("0.1.0")
        .author("Fabien Dubosson <fabien.dubosson@gmail.com>")
        .about("Manage your development workspaces with ease.")
        .get_matches();

    // Return the arguments
    matches
}

fn main() {
    let _ = parse_arguments();
}
