// External crates
extern crate ansi_term;
extern crate atty;
extern crate clap;
extern crate threadpool;
extern crate xdg_basedir;
extern crate yaml_rust;

// Project modules
mod commands;
mod context;
mod core;
mod vcs;

/// Run prevy.
fn main() {
    // Build the context:
    // - Handle command line arguments
    // - Handle environment variables
    // - Parse the configuration file
    // - Parse the workspace file
    let ctx = context::build_context();

    // Switch to workspace root
    ctx.workspace.cd_root();
}
