use context::{Context};

#[derive(Clone, Debug)]
pub struct Config {
    // TODO
}

impl Default for Config {
    fn default() -> Config {
        // TODO implement
        Config{}
    }
}

/// Get the context from default values and users preferences.
///
/// The order of priority is the following:
///
/// 1. Environment variables
/// 2. Command line parameters
/// 3. Workspace file
/// 4. User configuration file
pub fn parse_config(ctx: &mut Context) {
    // 4. User configuration file
    read_config_file(ctx);
    // 3. Workspace file
    read_workspace_file(ctx);
    // 2. Command line parameters
    read_args(ctx);
    // 1. Environment variables
    read_env(ctx);
}

fn read_config_file(ctx: &mut Context) {
    // TODO implement
}

fn read_workspace_file(ctx: &mut Context) {
    // TODO implement
}

fn read_args(ctx: &mut Context) {
    // TODO implement
}

fn read_env(ctx: &mut Context) {
    // TODO implement
}
