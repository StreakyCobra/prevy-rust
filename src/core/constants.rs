/// Default name of the configuration file within XDG_CONFIG_HOME.
pub const DEFAULT_CONFIGURATION_FILENAME: &'static str = "prevy.yaml";

/// Default name of the workspace file.
pub const DEFAULT_WORKSPACE_FILENAME: &'static str = ".prevy.yaml";

/// The prefix of prevy environment variables.
pub const VAR_PREFIX: &'static str = "PREVY_";

/// The section containing the configuration in the workspace file.
pub const SEC_WORKSPACE_CONFIG: &'static str = "config";

/// The section containing the repositories in the workspace file.
pub const SEC_WORKSPACE_REPOS: &'static str = "repos";

/// The section containing the repositories' remotes in the workspace file.
pub const SEC_WORKSPACE_REMOTES: &'static str = "remotes";

/// Identifier for the configuration filename.
pub const ID_CONFIGURATION_FILE: &'static str = "configuration_file";

/// Identifier for the workspace filename.
pub const ID_WORKSPACE_FILENAME: &'static str = "workspace_filename";

/// Identifier for the debuger configuration.
pub const ID_CONFIG_DEBUG: &'static str = "debug";

/// Identifier for the color configuration.
pub const ID_CONFIG_NOCOLOR: &'static str = "no-color";

pub fn var_to_id(var: String) -> String {
    var.trim_left_matches(VAR_PREFIX).to_lowercase().to_string()
}
