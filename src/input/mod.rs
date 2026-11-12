pub mod game;
pub mod mainmenu;
pub mod stats;

use crossterm::event::{Event, KeyEvent};
use std::time::Duration;

pub(super) fn read_key() -> std::io::Result<Option<KeyEvent>> {
    if crossterm::event::poll(Duration::from_millis(50))?
        && let Event::Key(key) = crossterm::event::read()?
    {
        return Ok(Some(key));
    }
    Ok(None)
}
