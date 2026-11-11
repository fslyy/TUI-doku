use std::time::Duration;

pub mod game;
pub mod mainmenu;
pub mod stats;
pub mod theme;

pub(crate) fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();

    let hours = secs / 3600;
    let mins = (secs % 3600) / 60;
    let secs = secs % 60;

    if hours > 0 {
        format!("{hours:02}:{mins:02}:{secs:02}")
    } else {
        format!("{mins:02}:{secs:02}")
    }
}