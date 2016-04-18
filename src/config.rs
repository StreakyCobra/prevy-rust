use clap::ArgMatches;

/// A struct storing the program configuration
pub struct Config<'a> {
    /// The command line arguments passed to the program
    pub args: ArgMatches<'a>,
    /// The name of
    pub workspace_file: &'a str,
}

/// Get the configuration from default values and users preferences.
pub fn get_config<'a>(args: ArgMatches<'a>) -> Config<'a> {
    Config { args: args, ..Default::default() }
}

impl<'a> Default for Config<'a> {
    fn default() -> Config<'a> {
        Config {
            args: ArgMatches::default(),
            workspace_file: ".projects.gws",
        }
    }
}
