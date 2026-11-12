use super::{SaveError, save_path};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct StatsSave {
    pub games_played: u32,
    pub total_time_played: u64,
    pub best_time: Option<u64>,
    pub average_time: Option<f64>,
}

impl StatsSave {
    pub fn new() -> Self {
        Self {
            games_played: 0,
            total_time_played: 0,
            best_time: None,
            average_time: None,
        }
    }

    pub fn update_stats(&mut self, time_played: u64) {
        if self.best_time.unwrap_or(10000) > time_played {
            self.best_time = Some(time_played);
        }
        if self.games_played > 0 {
            self.average_time = Some(
                (self.average_time.unwrap_or(0 as f64) * self.games_played as f64
                    + time_played as f64)
                    / (self.games_played + 1) as f64,
            )
        } else {
            self.average_time = Some(time_played as f64);
        }
        self.games_played += 1;
        self.total_time_played += time_played;
    }
}

pub fn update_and_save_stats(time_played: u64) -> Result<(), SaveError> {
    let path = save_path("stats.json".to_string());

    let mut stats = get_stats().unwrap_or_else(|_| StatsSave::new());

    stats.update_stats(time_played);

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
