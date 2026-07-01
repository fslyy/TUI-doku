use crate::board::Board;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
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

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct GameSave {
    pub board: Board,
    pub selected_row: usize,
    pub selected_col: usize,

    pub elapsed_seconds: u64,
}

impl GameSave {
    pub fn new(
        board: Board,
        selected_row: usize,
        selected_col: usize,
        elapsed_seconds: u64,
    ) -> Self {
        Self {
            board,
            selected_row,
            selected_col,
            elapsed_seconds,
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct StatsSave {
    pub games_played: u32,
    pub games_won: u32,
    pub total_time_played: u64,
    pub best_time: Option<u64>,
    pub current_streak: u32,
    pub longest_streak: u32,
    pub average_time: Option<f64>,
}

impl StatsSave {
    pub fn new() -> Self {
        Self {
            games_played: 0,
            games_won: 0,
            total_time_played: 0,
            best_time: None,
            current_streak: 0,
            longest_streak: 0,
            average_time: None,
        }
    }

    pub fn update_stats(&mut self, won: bool, time_played: u64) {
        self.games_played += 1;
        self.total_time_played += time_played;
        if won {
            self.games_won += 1;
            self.current_streak += 1;
        }
        if self.longest_streak < self.current_streak {
            self.longest_streak = self.current_streak;
        }
        if self.best_time.unwrap_or(10000) < time_played {
            self.best_time = Some(time_played);
        }
        self.average_time = Some((self.average_time.unwrap_or(0 as f64) + time_played as f64) / 2 as f64);
    }
}

fn save_path(name: String) -> PathBuf {
    let proj_dirs =
        ProjectDirs::from("dev", "fslyy", "tui-doku").expect("Failed to determine data directory");

    proj_dirs.data_dir().join(name)
}

pub fn save_game_state(save: &GameSave) -> Result<(), SaveError> {
    let path = save_path("savegame.json".to_string());

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(save)?;

    std::fs::write(path, json)?;

    Ok(())
}

pub fn load_game_state() -> Result<GameSave, SaveError> {
    let path = save_path("savegame.json".to_string());

    let json = std::fs::read_to_string(path)?;

    let save = serde_json::from_str(&json)?;

    Ok(save)
}

pub fn update_and_update_stats(won: bool, time_played: u64) -> Result<(), SaveError> {
    let path = save_path("stats.json".to_string());

    let mut stats = get_stats().unwrap_or_else(|_| StatsSave::new());

    stats.update_stats(won, time_played);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(&stats)?;

    std::fs::write(path, json)?;

    Ok(())
}

pub fn get_stats() -> Result<StatsSave, SaveError> {
    let path = save_path("stats.json".to_string());

    let json = std::fs::read_to_string(path)?;

    let stats = serde_json::from_str(&json)?;

    Ok(stats)
}
