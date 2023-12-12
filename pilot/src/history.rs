//! pilot history file

use std::path::PathBuf;

/// The name of the pilot history file
pub const CHISEL_HISTORY_FILE_NAME: &str = ".pilot_history";

/// Returns the path to foundry's global toml file that's stored at `~/.foundry/.pilot_history`
pub fn pilot_history_file() -> Option<PathBuf> {
    foundry_config::Config::foundry_dir().map(|p| p.join(CHISEL_HISTORY_FILE_NAME))
}
