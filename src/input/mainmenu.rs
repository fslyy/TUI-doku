use crate::app::{App, Screen};
use crate::save::game::load_game_state;
use crossterm::event::KeyCode;

pub fn handle_input(app: &mut App) -> std::io::Result<()> {
    if let Some(key) = super::read_key()? {
        match key.code {
            KeyCode::Char('c') if load_game_state().is_ok() => {
                app.load_game();
            }
            KeyCode::Char('n') => {
                app.start_game();
            }
            KeyCode::Char('s') => app.screen = Screen::Stats,
            KeyCode::Char('q') => {
                app.running = false;
            }
            _ => {}
        }
    }
    Ok(())
}
