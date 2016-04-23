pub mod config;
pub mod workspace;

// ------------------------------------------------------------------------- //
// Imports                                                                   //
// ------------------------------------------------------------------------- //

// Standard libraries imports
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

// External crates imports
use clap::ArgMatches;
use xdg_basedir::get_config_home;
use yaml_rust::Yaml;

// Project imports
use core::constants::*;
use self::config::{Config, parse_config};
use self::workspace::{Workspace, parse_workspace, find_workspace_root};
use utils::read_yaml_file;

// ------------------------------------------------------------------------- //
// Structure                                                                 //
// ------------------------------------------------------------------------- //

/// A structure storing the program context.
#[derive(Clone, Debug)]
pub struct Context<'a> {
    /// The program configuration.
    pub config: Config,
    /// The workspace content.
    pub workspace: Workspace,
    /// The command line arguments.
    args: ArgMatches<'a>,
    /// The environment variables.
    env_vars: HashMap<String, String>,
    /// The path to the configuration file.
    configuration_file: Option<String>,
    /// The content of the configuration file.
    configuration_file_content: Yaml,
    /// The workspace file
    workspace_file: String,
    /// The default name of the workspace file.
    workspace_filename: String,
    /// The content of the workspace file.
    workspace_file_content: Yaml,
}

/// Define the default values for the context.
impl<'a> Default for Context<'a> {
    fn default() -> Context<'a> {
        Context {
            config: Config::default(),
            workspace: Workspace::default(),
            args: ArgMatches::default(),
            env_vars: Default::default(),
            configuration_file: match get_config_home() {
                Err(_) => None,
                Ok(val) => {
                    match val.join(DEFAULT_CONFIGURATION_FILENAME).to_str() {
                        None => None,
                        Some(val) => Some(val.to_string()),
                    }
                }
            },
            configuration_file_content: Yaml::Null,
            workspace_file: String::default(),
            workspace_filename: DEFAULT_WORKSPACE_FILENAME.to_string(),
            workspace_file_content: Yaml::Null,
        }
    }
}

// ------------------------------------------------------------------------- //
// Public API                                                                //
// ------------------------------------------------------------------------- //

/// Build the context of the application.
///
/// The context is built out of the command line arguments and the
/// configurations files. The workspace is also extracted to the context.
pub fn build_context(args: ArgMatches) -> Context {
    // First bootstrap the context
    let mut ctx = bootstrap_context(args);
    // Second parse the workspace
    parse_workspace(&mut ctx);
    // Third parse the different configurations
    parse_config(&mut ctx);
    // Return the built context
    ctx
}

// ------------------------------------------------------------------------- //
// Internal functions                                                        //
// ------------------------------------------------------------------------- //

/// Bootstrap the context.
///
/// This parse the environment variables, the command line arguments and the
/// configuration file in order to select the correct configuration and
/// workspace files.
fn bootstrap_context(args: ArgMatches) -> Context {
    // First create a default context
    let mut ctx = Context {
        args: args,
        env_vars: env::vars().filter_map(|s| parse_prevy_var(s)).collect(),
        ..Default::default()
    };
    // Set the configuration file to use. Can only be overidden by a command
    // line argument or an environment variable.
    match ctx.env_vars.get(ID_CONFIGURATION_FILE) {
        None => (),
        Some(filepath) => ctx.configuration_file = Some(filepath.to_string()),
    }
    match ctx.args.value_of(ID_CONFIGURATION_FILE) {
        None => (),
        Some(filepath) => ctx.configuration_file = Some(filepath.to_string()),
    }
    // Read the content of the configuration file and store it for performance
    match ctx.configuration_file.clone() {
        None => (),
        Some(filepath) => {
            ctx.configuration_file_content = read_yaml_file(filepath).unwrap_or(Yaml::Null)
        }
    }
    // Set the workspace filename to use, can only be overidden by the
    // configuration file, an environment variable or a command line argument.
    match ctx.configuration_file_content[ID_WORKSPACE_FILENAME].as_str() {
        None => (),
        Some(filename) => ctx.workspace_filename = filename.to_string(),
    }
    match ctx.env_vars.get(ID_WORKSPACE_FILENAME) {
        None => (),
        Some(filename) => ctx.workspace_filename = filename.to_string(),
    }
    match ctx.args.value_of(ID_WORKSPACE_FILENAME) {
        None => (),
        Some(filename) => ctx.workspace_filename = filename.to_string(),
    }
    // Read the content of the workspace file and store it for performance
    ctx.workspace.root = find_workspace_root(ctx.workspace_filename.clone());
    ctx.workspace_file = PathBuf::from(ctx.workspace.root.clone())
                             .join(ctx.workspace_filename.clone())
                             .to_str()
                             .unwrap()
                             .to_string();
    match read_yaml_file(ctx.workspace_file.clone()) {
        Ok(content) => ctx.workspace_file_content = content,
        Err(error) => error.exit(),
    }
    // Return the bootstrapped context that is ready to be parsed
    ctx
}

/// Parse an environment variables.
fn parse_prevy_var(val: (String, String)) -> Option<(String, String)> {
    if val.0.starts_with(VAR_PREFIX) {
        Some((var_to_id(val.0), val.1))
    } else {
        None
    }
}
