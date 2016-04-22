use std::env;
use std::fs::File;
use std::io::prelude::*;

use clap::ArgMatches;
use xdg_basedir::get_config_home;
use yaml_rust::{Yaml, YamlLoader};

use config::{Config, parse_config};
use errors::{Error, ErrorKind, Result};
use workspace::Workspace;

const DEFAULT_CONFIGURATION_FILE: &'static str = "prevy.yaml";
const DEFAULT_WORKSPACE_FILENAME: &'static str = ".prevy.yaml";

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

pub fn build_context(args: ArgMatches) -> Context {
    // First bootstrap the context
    let mut ctx = bootstrap_context(args);
    // Then parse the configurations
    parse_config(&mut ctx);
    // Return the built context
    ctx
}

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

fn read_yaml_file(filename: String) -> Result<Yaml> {
    let mut file = match File::open(filename.clone()) {
        Ok(file) => file,
        Err(error) => {
            return Err(Error {
                kind: ErrorKind::IO,
                message: format!{"Error when trying to open '{}'.", filename}.to_string(),
                error: Some(error.to_string()),
            })
        }
    };

    let mut content = String::new();

    let _ = file.read_to_string(&mut content);

    match YamlLoader::load_from_str(&content) {
        Err(error) => {
            Err(Error {
                kind: ErrorKind::Parse,
                message: format!{"Error while parsing '{}'", filename}.to_string(),
                error: Some(error.to_string()),
            })
        }
        Ok(yaml) => Ok(yaml[0].clone()),
    }
}

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
