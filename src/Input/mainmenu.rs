use crate::app::{App, Screen};
use crate::save::load_game_state;
use crossterm::event::{
    self,
    Event::{self, Key},
    KeyCode,
};
use std::time::Duration;

pub fn handle_input(app: &mut App) -> std::io::Result<()> {
    if event::poll(Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('c') => {
                    if load_game_state().is_ok() {
                        app.load_game();
                    }
                }
                KeyCode::Char('n') => {
                    app.start_game();
                }
                KeyCode::Char('q') => {
                    app.running = false;
                }
                _ => {}
            }
        }
    }
    Ok(())
}
