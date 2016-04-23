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
    // Build the context
    let ctx = context::build_context();

    // Print the context if debug is enabled
    if ctx.config.debug {
        core::display::debug(&format!("{:#?}", ctx));
    }

    // Starting from here, we will work from workspace root
    ctx.workspace.cd_root();
}
