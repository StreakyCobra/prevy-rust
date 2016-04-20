use clap::ArgMatches;

/// A struct storing the program configuration.
pub struct Config<'a> {
    /// The command line arguments passed to the program.
    pub args: ArgMatches<'a>,
    /// The name of the workspace file
    pub workspace_file: String,
    /// The name of the workspace file
    pub config_file: String,
}

/// Get the configuration from default values and users preferences.
///
/// The order of priority is the following:
///
/// 1. Environment variables
/// 2. Command line parameters
/// 3. Workspace file
/// 4. User configuration file
/// 5. Default software values
pub fn read_config(args: ArgMatches) -> Config {
    let mut conf = Config { args: args, ..Default::default() };
    read_args(&mut conf);
    read_config_file(&mut conf);
    conf
}

fn read_args(conf: &mut Config) {
    // TODO Replace by real code
    match conf.args.value_of("workspace_file") {
        None => (),
        Some(val) => conf.workspace_file = val.to_string(),
    }
}

fn read_config_file(conf: &mut Config) {
    // TODO Replace by real code
    match conf.args.value_of("workspace_file") {
        None => (),
        Some(val) => conf.workspace_file = val.to_string(),
    }
}

impl<'a> Default for Config<'a> {
    fn default() -> Config<'a> {
        Config {
            args: ArgMatches::default(),
            config_file: "~/.config/prevy.yml".to_string(),
            workspace_file: ".prevy.yml".to_string(),
        }
    }
}
