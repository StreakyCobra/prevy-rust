use std::env;
use std::fs::File;
use std::io::prelude::*;

use clap::ArgMatches;
use xdg_basedir::get_config_home;
use yaml_rust::{Yaml, YamlLoader};

use errors::{Error, ErrorKind, Result};

const DEFAULT_CONFIGURATION_FILE: &'static str = "prevy.yaml";
const DEFAULT_WORKSPACE_FILENAME: &'static str = ".prevy.yaml";

/// A struct storing the program configuration.
#[derive(Clone, Debug)]
pub struct Config<'a> {
    /// The command line arguments.
    pub args: ArgMatches<'a>,
    /// The path to the configuration file.
    pub configuration_file: Option<String>,
    /// The content of the configuration file.
    pub configuration_file_content: Yaml,
    /// The default name of the workspace file.
    pub workspace_filename: String,
    /// The content of the workspace file.
    pub workspace_file_content: Yaml,
}

/// Get the configuration from default values and users preferences.
///
/// The order of priority is the following:
///
/// 1. Environment variables
/// 2. Command line parameters
/// 3. Workspace file
/// 4. User configuration file
/// 5. Default configuration values
pub fn parse_config(args: ArgMatches) -> Config {
    // 5. Default configuration values
    let mut conf = bootstrap_config(args);
    // 4. User configuration file
    read_config_file(&mut conf);
    // 3. Workspace file
    read_workspace_file(&mut conf);
    // 2. Command line parameters
    read_args(&mut conf);
    // 1. Environment variables
    read_env(&mut conf);
    // Return the parsed configuration
    conf
}

fn bootstrap_config(args: ArgMatches) -> Config {
    // First create a default configuration
    let mut conf = Config { args: args, ..Default::default() };
    // Set the configuration file to use, can only be overidden by a command
    // line argument or an environment variable.
    match conf.args.value_of("configuration_file") {
        None => (),
        Some(filepath) => conf.configuration_file = Some(filepath.to_string()),
    }
    match env::var("PREVY_CONFIGURATION_FILE") {
        Err(_) => (),
        Ok(filepath) => conf.configuration_file = Some(filepath.to_string()),
    }
    // Read the content of the configuration file
    match conf.configuration_file.clone() {
        None => (),
        Some(filepath) => {
            conf.configuration_file_content = read_yaml_file(filepath).unwrap_or(Yaml::Null)
        }
    }
    // Set the workspace filename to use, can only be overidden by the
    // configuration file, a command line argument or an environment variable.
    match conf.configuration_file_content["workspace_filename"].as_str() {
        None => (),
        Some(filename) => conf.workspace_filename = filename.to_string(),
    }
    match conf.args.value_of("workspace_filename") {
        None => (),
        Some(filename) => conf.workspace_filename = filename.to_string(),
    }
    match env::var("PREVY_WORKSPACE_FILENAME") {
        Err(_) => (),
        Ok(filename) => conf.workspace_filename = filename.to_string(),
    }
    // Return the bootstrapped configuration that is ready to be parsed
    conf
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
                kind: ErrorKind::Parser,
                message: format!{"Error while parsing '{}'", filename}.to_string(),
                error: Some(error.to_string()),
            })
        }
        Ok(yaml) => Ok(yaml[0].clone()),
    }
}

fn read_config_file(conf: &mut Config) {
    // TODO
}

fn read_workspace_file(conf: &mut Config) {
    // TODO
}

fn read_args(conf: &mut Config) {
    // TODO
}

fn read_env(conf: &mut Config) {
    // TODO
}

impl<'a> Default for Config<'a> {
    fn default() -> Config<'a> {
        Config {
            args: ArgMatches::default(),
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
