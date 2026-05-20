use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

use crate::{app::App, board::Board};

// INPUT LOGIC
pub fn handle_input(app: &mut App) -> std::io::Result<()> {
    if event::poll(Duration::from_millis(50))? {
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
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    let digit = c.to_digit(10).unwrap() as u8;

                    if (1..=9).contains(&digit) {
                        let cell = Board::get_mut(
                            &mut app.board,
                            app.selected_row,
                            app.selected_col,
                        );
                        if !cell.fixed {
                            cell.value = Some(digit);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}