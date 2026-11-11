use crate::app::{App, Screen};
use crossterm::event::KeyCode;

pub fn handle_input(app: &mut App) -> std::io::Result<()> {
    if let Some(key) = super::read_key()? {
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
