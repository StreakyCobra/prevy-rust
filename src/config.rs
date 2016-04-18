use clap::ArgMatches;

pub struct Config<'a> {
    pub workspace_file: &'a str,
    pub args: Option<ArgMatches<'a>>,
}

pub fn get_config<'a>() -> Config<'a> {
    Config { ..Default::default() }
}

impl<'a> Default for Config<'a> {
    fn default() -> Config<'a> {
        Config {
            workspace_file: ".projects.gws",
            args: None,
        }
    }
}
