use std::fs::File;
use std::io::prelude::*;

use clap::ArgMatches;
use xdg_basedir::get_config_home;
use yaml_rust::YamlLoader;

use errors::{Error, ErrorKind};

const DEFAULT_CONFIGURATION_FILE_NAME: &'static str = "prevy.yaml";
const DEFAULT_WORKSPACE_FILE_NAME: &'static str = ".prevy.yaml";

/// A struct storing the program configuration.
pub struct Config<'a> {
    /// The command line arguments.
    pub args: ArgMatches<'a>,
    /// The path to the workspace file.
    pub workspace_file: String,
    /// The path to the configuration file.
    pub config_file: Option<String>,
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
    let mut conf = Config { args: args, ..Default::default() };
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

fn read_config_file(conf: &mut Config) {
    match conf.config_file.clone() {
        None => (),
        Some(filename) => {
            let mut file = match File::open(filename) {
                Ok(file) => file,
                Err(_) => {
                    Error {
                        kind: ErrorKind::IO,
                        message: "Error when trying to open the configuration file.".to_string(),
                    }
                    .exit()
                }
            };

            let mut content = String::new();

            let _ = file.read_to_string(&mut content);

            match YamlLoader::load_from_str(&content) {
                Err(_) => {
                    Error {
                        kind: ErrorKind::Parser,
                        message: "Error while parsing the YAML configuration file.".to_string(),
                    }
                    .exit()
                }
                Ok(yaml) => {
                    println!("{:?}", yaml);
                    ()
                }
            }
        }
    };
}

fn read_workspace_file(conf: &mut Config) {
    // TODO
}

fn read_args(conf: &mut Config) {
    match conf.args.value_of("workspace_file") {
        None => (),
        Some(val) => conf.workspace_file = val.to_string(),
    }
    match conf.args.value_of("config_file") {
        None => (),
        Some(val) => conf.config_file = Some(val.to_string()),
    }
}

fn read_env(conf: &mut Config) {
    // TODO
}

impl<'a> Default for Config<'a> {
    fn default() -> Config<'a> {
        Config {
            args: ArgMatches::default(),
            config_file: match get_config_home() {
                Err(_) => None,
                Ok(val) => {
                    match val.join(DEFAULT_CONFIGURATION_FILE_NAME).to_str() {
                        None => None,
                        Some(val) => Some(val.to_string()),
                    }
                }
            },
            workspace_file: DEFAULT_WORKSPACE_FILE_NAME.to_string(),
        }
    }
}
