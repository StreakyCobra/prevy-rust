use clap::ArgMatches;

/// A struct storing the program configuration.
pub struct Config<'a> {
    /// The command line arguments passed to the program.
    pub args: ArgMatches<'a>,
    /// The name of the workspace file
    pub workspace_file: &'a str,
    /// The name of the workspace file
    pub config_file: &'a str,
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
pub fn get_config<'a>(args: ArgMatches<'a>) -> Config<'a> {
    // First get default software values
    let mut conf: Config<'a> = Config { args: args, ..Default::default() };
    // Parse the command line arguments
    read_args(&mut conf);
    // Return the final config
    conf
}

fn read_args<'a>(conf: &'a mut Config) {
    // TODO Not working when using `Config<'a>` instead of `Config` in function signature
}

impl<'a> Default for Config<'a> {
    fn default() -> Config<'a> {
        Config {
            args: ArgMatches::default(),
            config_file: "~/.config/prevy.yml",
            workspace_file: ".prevy.yml",
        }
    }
}
