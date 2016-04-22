/// Default name of the configuration file within XDG_CONFIG_HOME
pub const DEFAULT_CONFIGURATION_FILENAME: &'static str = "prevy.yaml";

/// Default name of the workspace file
pub const DEFAULT_WORKSPACE_FILENAME: &'static str = ".prevy.yaml";

/// Text identifier for the configuration filename
pub const ID_CONFIGURATION_FILENAME: &'static str = "configuration_filename";

/// Text identifier for the workspace filename
pub const ID_WORKSPACE_FILENAME: &'static str = "workspace_filename";

/// Text identifier for the debuger configuration
pub const ID_CONFIG_DEBUG: &'static str = "debug";

pub fn id_to_var(id: &str) -> String {
    "PREVY_".to_string() + &id.to_uppercase()
}
