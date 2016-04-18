use clap::ArgMatches;

pub struct Config<'a> {
    pub args: Option<ArgMatches<'a>>,
    pub workspace_file: &'a str,
}

pub fn get_config<'a>() -> Config<'a> {
    Config { ..Default::default() }
}

impl<'a> Default for Config<'a> {
    fn default() -> Config<'a> {
        Config {
            args: None,
            workspace_file: ".projects.gws",
        }
    }
}
