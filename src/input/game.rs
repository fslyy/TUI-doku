use crate::{
    app::{App, Screen},
    board::{Board, check::is_valid_num},
};
use crossterm::event::KeyCode;

pub fn handle_input(app: &mut App) -> std::io::Result<()> {
    if let Some(key) = super::read_key()? {
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
                    let cell = Board::get_mut(&mut app.board, app.selected_row, app.selected_col);

                    if !cell.fixed {
                        cell.value = None;
                        cell.is_valid = true;
                    }
                }
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    let digit = c.to_digit(10).unwrap() as u8;
                    if (1..=9).contains(&digit) {
                        on_digit(app, c);
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
                    } else {
                        app.clear_saved_game()
                    }
                    app.running = false;
                }
                KeyCode::Esc => {
                    if !app.win {
                        app.save_game();
                    } else {
                        app.clear_saved_game()
                    }
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

fn on_digit(app: &mut App, c: char) {
    let digit = c.to_digit(10).unwrap() as u8;

    if app.board.cells[app.selected_row][app.selected_col].fixed {
        return;
    }

    if app.notes_mode {
        let cell = app.board.get_mut(app.selected_row, app.selected_col);

        cell.notes[(digit - 1) as usize] = !cell.notes[(digit - 1) as usize];
    } else {
        let mut board = app.board;
        let prev_digit = board.cells[app.selected_row][app.selected_col]
            .value
            .unwrap_or(0);
        board.cells[app.selected_row][app.selected_col].value = None;

        let cell = app.board.get_mut(app.selected_row, app.selected_col);

        if prev_digit == digit {
            cell.value = None;
            cell.is_valid = true;
        } else {
            let is_valid = is_valid_num(&board, digit, app.selected_row, app.selected_col);
            cell.value = Some(digit);
            cell.is_valid = is_valid;

            if Board::is_complete(&app.board) {
                app.check_win();
            }
        }
    }
}
