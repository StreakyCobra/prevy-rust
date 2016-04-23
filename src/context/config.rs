// ------------------------------------------------------------------------- //
// Imports                                                                   //
// ------------------------------------------------------------------------- //

// Standard libraries imports
use std::str::FromStr;

// External crates imports
use yaml_rust::Yaml;

// Project imports
use core::constants::*;
use context::Context;

// ------------------------------------------------------------------------- //
// Structures                                                                //
// ------------------------------------------------------------------------- //

/// A structure storing the program configuration.
#[derive(Clone, Debug)]
pub struct Config {
    /// Print debug information.
    pub debug: bool,
    /// Disable colored output.
    pub nocolor: bool,
}

/// Define the default values for the config.
impl Default for Config {
    fn default() -> Config {
        Config {
            debug: false,
            nocolor: false,
        }
    }
}

// ------------------------------------------------------------------------- //
// Macros                                                                    //
// ------------------------------------------------------------------------- //

macro_rules! eval_arg_bool {
    ($ctx:expr, $elem:expr, $id:expr) => (
        match $ctx.args.is_present($id) {
            true => $elem = true,
            false => (),
        }
    )
}

macro_rules! eval_env_bool {
    ($ctx:expr, $elem:expr, $id:expr) => (
        match $ctx.env_vars.get($id) {
            None => (),
            Some(val) => {
                match bool::from_str(val) {
                    Err(_) => (),
                    Ok(val) => $elem = val,
                }
            }
        }
    )
}

macro_rules! eval_yaml_bool {
    ($yaml:expr, $elem:expr, $id:expr) => (
        match $yaml[$id].as_bool() {
            None => (),
            Some(val) => $elem = val,
        }
    )
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
    eval_arg_bool!(ctx, ctx.config.debug, ID_CONFIG_DEBUG);
    eval_arg_bool!(ctx, ctx.config.nocolor, ID_CONFIG_NOCOLOR);
}

fn read_env(ctx: &mut Context) {
    eval_env_bool!(ctx, ctx.config.debug, ID_CONFIG_DEBUG);
    eval_env_bool!(ctx, ctx.config.nocolor, ID_CONFIG_NOCOLOR);
}

fn read_yaml_config(ctx: &mut Context, yaml: Yaml) {
    eval_yaml_bool!(yaml, ctx.config.debug, ID_CONFIG_DEBUG);
    eval_yaml_bool!(yaml, ctx.config.nocolor, ID_CONFIG_NOCOLOR);
}
