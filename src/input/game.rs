use crate::app::{App, Screen};
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub fn handle_input(app: &mut App) -> std::io::Result<()> {
    if event::poll(Duration::from_millis(50))?
        && let Event::Key(key) = event::read()?
    {
        if app.game_paused {
            match key.code {
                KeyCode::Char('p') | KeyCode::Esc => {
                    app.pause_game();
                }
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Up if app.selected_row > 0 => {
                    app.selected_row -= 1;
                }
                KeyCode::Down if app.selected_row < 8 => {
                    app.selected_row += 1;
                }
                KeyCode::Left if app.selected_col > 0 => {
                    app.selected_col -= 1;
                }
                KeyCode::Right if app.selected_col < 8 => {
                    app.selected_col += 1;
                }
                KeyCode::Backspace => {
                    app.on_backspace();
                }
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    let digit = c.to_digit(10).unwrap() as u8;
                    if (1..=9).contains(&digit) {
                        app.on_digit(c);
                    }
                }
                KeyCode::Char('n') => {
                    app.notes_mode = !app.notes_mode;
                }
                KeyCode::Char('p') => {
                    app.pause_game();
                }
                KeyCode::Char('q') => {
                    if !app.win {
                        app.save_game();
                    }
                    app.running = false;
                }
                KeyCode::Esc => {
                    app.screen = Screen::MainMenu;
                }
                KeyCode::Enter if app.win => {
                    app.start_game();
                }
                _ => {}
            }
        }
    }
    Ok(())
}
