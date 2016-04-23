/// Default name of the configuration file within XDG_CONFIG_HOME.
pub const DEFAULT_CONFIGURATION_FILENAME: &'static str = "prevy.yaml";

/// Default name of the workspace file.
pub const DEFAULT_WORKSPACE_FILENAME: &'static str = ".prevy.yaml";

/// The prefix of prevy environment variables.
pub const VAR_PREFIX: &'static str = "PREVY_";

/// Text identifier for the configuration filename.
pub const ID_CONFIGURATION_FILE: &'static str = "configuration_file";

/// Text identifier for the workspace filename.
pub const ID_WORKSPACE_FILENAME: &'static str = "workspace_filename";

/// Text identifier for the debuger configuration.
pub const ID_CONFIG_DEBUG: &'static str = "debug";

pub fn id_to_var(id: &str) -> String {
    VAR_PREFIX.to_string() + &id.to_uppercase()
}
