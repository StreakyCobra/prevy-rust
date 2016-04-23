// External crates
extern crate ansi_term;
extern crate atty;
extern crate clap;
extern crate xdg_basedir;
extern crate yaml_rust;

// Project modules
mod context;
mod core;
mod utils;

/// Run prevy.
fn main() {
    // Build the context (arguments, environment variables, config, workspace)
    let ctx = context::build_context();

    // Starting from here, we will work from workspace root
    ctx.workspace.cd_root();
}
