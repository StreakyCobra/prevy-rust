// ------------------------------------------------------------------------- //
// Imports                                                                   //
// ------------------------------------------------------------------------- //

// Standard libraries imports
use std::env;

// External crates imports
use yaml_rust::Yaml;

// Project imports
use constants::*;
use context::Context;

// ------------------------------------------------------------------------- //
// Structure                                                                 //
// ------------------------------------------------------------------------- //

/// A structure storing the program configuration.
#[derive(Clone, Debug)]
pub struct Config {
    /// Print debug information.
    pub debug: bool,
}

/// Define the default values for the config.
impl Default for Config {
    fn default() -> Config {
        Config { debug: false }
    }
}

// ------------------------------------------------------------------------- //
// Public API                                                                //
// ------------------------------------------------------------------------- //

/// Get the context from default values and users preferences.
///
/// The order of priority is the following:
///
/// 1. Command line parameters
/// 2. Environment variables
/// 3. Workspace file
/// 4. User configuration file
pub fn parse_config(ctx: &mut Context) {
    // 4. User configuration file
    read_config_file(ctx);
    // 3. Workspace file
    read_workspace_file(ctx);
    // 2. Environment variables
    read_env(ctx);
    // 1. Command line parameters
    read_args(ctx);
}

// ------------------------------------------------------------------------- //
// Internal functions                                                        //
// ------------------------------------------------------------------------- //

fn read_config_file(ctx: &mut Context) {
    let yaml = ctx.configuration_file_content.clone();
    read_yaml_config(ctx, yaml);
}

fn read_workspace_file(ctx: &mut Context) {
    let yaml = ctx.workspace_file_content["config"].clone();
    read_yaml_config(ctx, yaml);
}

fn read_args(ctx: &mut Context) {
    match ctx.args.is_present(ID_CONFIG_DEBUG) {
        true => ctx.config.debug = true,
        false => (),
    }
}

fn read_env(ctx: &mut Context) {
    match env::var(id_to_var(ID_CONFIG_DEBUG)) {
        Err(_) => (),
        Ok(_) => ctx.config.debug = true,
    }
}

fn read_yaml_config(ctx: &mut Context, yaml: Yaml) {
    match yaml[ID_CONFIG_DEBUG].as_bool() {
        None => (),
        Some(val) => ctx.config.debug = val,
    }
}
