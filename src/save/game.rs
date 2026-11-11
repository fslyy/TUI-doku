use crate::board::Board;
use serde::{Deserialize, Serialize};
use std::fs;
use super::{SaveError, save_path};

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

pub fn save_game_state(save: &GameSave) -> Result<(), SaveError> {
    let path = save_path("savegame.json".to_string());

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(save)?;

    std::fs::write(path, json)?;

    Ok(())
}

pub fn clear_game_state() -> Result<(), SaveError> {
    let path = save_path("savegame.json".to_string());

    if path.exists() {
        std::fs::remove_file(path)?;
    }

    Ok(())
}

pub fn load_game_state() -> Result<GameSave, SaveError> {
    let path = save_path("savegame.json".to_string());

    let json = std::fs::read_to_string(path)?;

    let save = serde_json::from_str(&json)?;

    Ok(save)
}