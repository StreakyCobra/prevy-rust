// External crates
extern crate ansi_term;
extern crate atty;
extern crate clap;
extern crate xdg_basedir;
extern crate yaml_rust;

// Project modules
mod context;
mod core;
mod vcs;

/// Run prevy.
fn main() {
    // Build the context:
    // - Handle command line arguments
    // - Handle environment variables
    // - Parse the configuration
    // - Parse the workspace
    // - Prepare a display
    let ctx = context::build_context();

    // Switch to workspace root
    ctx.workspace.cd_root();
}
