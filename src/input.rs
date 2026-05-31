use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

use crate::{app::App, board::Board, app::Screen, board::generate_board};

// INPUT LOGIC
pub fn handle_input(app: &mut App) -> std::io::Result<()> {
    if event::poll(Duration::from_millis(50))? {
        match app.screen {
            Screen::MainMenu => {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('n') => {
                            let (board, solution) = generate_board();
                            app.board = board;
                            app.solution = solution;
                            app.screen = Screen::Game;
                            app.start_time = std::time::Instant::now();
                        }
                        KeyCode::Char('q') => {
                            app.running = false;
                        }
                        _ => {}
                    }
                }
            }
            Screen::Game => {
    if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    app.running = false;
                }
                KeyCode::Up => {
                    if app.selected_row > 0 {
                        app.selected_row -= 1;
                    }
                }
                KeyCode::Down => {
                    if app.selected_row < 8 {
                        app.selected_row += 1;
                    }
                }
                KeyCode::Left => {
                    if app.selected_col > 0 {
                        app.selected_col -= 1;
                    }
                }
                KeyCode::Right => {
                    if app.selected_col < 8 {
                        app.selected_col += 1;
                    }
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
                    app.notes = !app.notes;
                }
                _ => {}
            }
        }
    }
}
    }

    Ok(())
}