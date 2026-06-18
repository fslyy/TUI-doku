use serde::{Deserialize, Serialize};
use std::fs;
use crate::board::Board;
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

impl GameSave{
    pub fn new(board: Board, selected_row: usize, selected_col: usize, elapsed_seconds: u64) -> Self {
        Self { board, selected_row, selected_col, elapsed_seconds}
    }
}

pub fn save_game_state(save: &GameSave) -> Result<(), SaveError> {
    let json = serde_json::to_string_pretty(save)?;
    
    std::fs::write("savegame.json", json)?;
    
    Ok(())
}

pub fn load_game_state() -> Result<GameSave, SaveError> {
    let json = std::fs::read_to_string("savegame.json")?;
    
    let save = serde_json::from_str(&json)?;
    
    Ok(save)
}