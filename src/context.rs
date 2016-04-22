use std::env;

use clap::ArgMatches;
use xdg_basedir::get_config_home;
use yaml_rust::Yaml;

use config::{Config, parse_config};
use workspace::Workspace;
use utils::read_yaml_file;

const DEFAULT_CONFIGURATION_FILE: &'static str = "prevy.yaml";
const DEFAULT_WORKSPACE_FILENAME: &'static str = ".prevy.yaml";

// ------------------------------------------------------------------------- //
// Structure                                                                 //
// ------------------------------------------------------------------------- //

/// A struct storing the program context.
#[derive(Clone, Debug)]
pub struct Context<'a> {
    /// The command line arguments.
    pub args: ArgMatches<'a>,
    /// The program configuration.
    pub config: Config,
    /// The workspace information.
    pub workspace: Workspace,
    /// The path to the configuration file.
    pub configuration_file: Option<String>,
    /// The content of the configuration file.
    pub configuration_file_content: Yaml,
    /// The default name of the workspace file.
    pub workspace_filename: String,
    /// The content of the workspace file.
    pub workspace_file_content: Yaml,
}

/// Implementing the default values for the context.
impl<'a> Default for Context<'a> {
    fn default() -> Context<'a> {
        Context {
            args: ArgMatches::default(),
            config: Config::default(),
            workspace: Workspace::default(),
            configuration_file: match get_config_home() {
                Err(_) => None,
                Ok(val) => {
                    match val.join(DEFAULT_CONFIGURATION_FILE).to_str() {
                        None => None,
                        Some(val) => Some(val.to_string()),
                    }
                }
            },
            configuration_file_content: Yaml::Null,
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
    // Then parse the configurations
    parse_config(&mut ctx);
    // Return the built context
    ctx
}

// ------------------------------------------------------------------------- //
// Internal functions                                                        //
// ------------------------------------------------------------------------- //

/// Bootstrap the context.
///
/// This parse the command line arguments, the environment variables and the
/// configuration file in order to select the correct configuration and
/// workspace files.
fn bootstrap_context(args: ArgMatches) -> Context {
    // First create a default context
    let mut ctx = Context { args: args, ..Default::default() };
    // Set the configuration file to use, can only be overidden by a command
    // line argument or an environment variable.
    match ctx.args.value_of("configuration_file") {
        None => (),
        Some(filepath) => ctx.configuration_file = Some(filepath.to_string()),
    }
    match env::var("PREVY_CONFIGURATION_FILE") {
        Err(_) => (),
        Ok(filepath) => ctx.configuration_file = Some(filepath.to_string()),
    }
    // Read the content of the configuration file
    match ctx.configuration_file.clone() {
        None => (),
        Some(filepath) => {
            ctx.configuration_file_content = read_yaml_file(filepath).unwrap_or(Yaml::Null)
        }
    }
    // Set the workspace filename to use, can only be overidden by the
    // configuration file, a command line argument or an environment variable.
    match ctx.configuration_file_content["workspace_filename"].as_str() {
        None => (),
        Some(filename) => ctx.workspace_filename = filename.to_string(),
    }
    match ctx.args.value_of("workspace_filename") {
        None => (),
        Some(filename) => ctx.workspace_filename = filename.to_string(),
    }
    match env::var("PREVY_WORKSPACE_FILENAME") {
        Err(_) => (),
        Ok(filename) => ctx.workspace_filename = filename.to_string(),
    }
    // Return the bootstrapped context that is ready to be parsed
    ctx
}
