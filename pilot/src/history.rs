//! pilot history file

use std::path::PathBuf;

/// The name of the pilot history file
pub const CHISEL_HISTORY_FILE_NAME: &str = ".pilot_history";

/// Returns the path to orbitalis's global toml file that's stored at `~/.orbitalis/.pilot_history`
pub fn pilot_history_file() -> Option<PathBuf> {
    orbitalis_config::Config::orbitalis_dir().map(|p| p.join(CHISEL_HISTORY_FILE_NAME))
}
