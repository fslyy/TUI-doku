pub mod board;
pub mod pause;
pub mod sidebar;

use ratatui::prelude::*;

use crate::app::App;

pub(super) const CELL_WIDTH: u16 = 7;
pub(super) const CELL_HEIGHT: u16 = 4;

pub(super) const NOTE_POSITIONS: [(u16, u16); 9] = [
    (1, 1),
    (3, 1),
    (5, 1),
    (1, 2),
    (3, 2),
    (5, 2),
    (1, 3),
    (3, 3),
    (5, 3),
];

#[derive(Clone, Copy)]
struct CellData {
    value: Option<u8>,
    notes: [bool; 9],
    selected: bool,
    fixed: bool,
    is_valid: bool,
}

pub fn render(frame: &mut Frame, app: &App) {
    // Gesamtbreite der UI (Board + Sidebar)
    let content = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(90),
            Constraint::Fill(1),
        ])
        .split(frame.area());

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(70), Constraint::Length(20)])
        .split(content[1]);

    board::render_board(frame, app, chunks[0]);
    sidebar::render_sidebar(frame, app, chunks[1]);

    if app.game_paused {
        pause::render_pause_overlay(frame, app);
    }
}
