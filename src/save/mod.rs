pub mod game;
pub mod stats;

use directories::ProjectDirs;
use std::path::PathBuf;
use std::{fmt, io};

#[derive(Debug)]
pub enum SaveError {
    Io(io::Error),
    InvalidFormat(serde_json::Error),
}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SaveError::Io(e) => write!(f, "Failed to read save file: {}", e),
            SaveError::InvalidFormat(e) => write!(f, "Invalid save file: {}", e),
        }
    }
}

impl From<io::Error> for SaveError {
    fn from(err: io::Error) -> Self {
        SaveError::Io(err)
    }
}

impl From<serde_json::Error> for SaveError {
    fn from(err: serde_json::Error) -> Self {
        SaveError::InvalidFormat(err)
    }
}

pub(super) fn save_path(name: String) -> PathBuf {
    let proj_dirs =
        ProjectDirs::from("dev", "fslyy", "tui-doku").expect("Failed to determine data directory");

    proj_dirs.data_dir().join(name)
}
