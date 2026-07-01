use std::time::Duration;

use crate::app::{App, Screen};
use crossterm::event::{
    self,
    Event::{self},
    KeyCode,
};

pub fn handle_input(app: &mut App) -> std::io::Result<()> {
    if event::poll(Duration::from_millis(50))?
        && let Event::Key(key) = event::read()?
    {
        match key.code {
            KeyCode::Esc => {
                app.screen = Screen::MainMenu;
            }
            KeyCode::Char('q') => {
                app.running = false;
            }
            _ => {}
        }
    }
    Ok(())
}
