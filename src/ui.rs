use ratatui::{
    buffer::{Buffer, Cell},
    layout::Rect,
    prelude::*,
};

use crate::app::{self, App};
use std::time::Duration;

#[derive(Clone)]
pub struct Theme {
    pub grid: Color,

    pub bg_light: Color,
    pub bg_dark: Color,
    pub bg_grid_highlight: Color,
    pub bg_num_highlight: Color,
    pub bg_selected: Color,
    pub bg_invalid: Color,

    pub number_fixed: Color,
    pub number_user: Color,

    pub note: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            grid: Color::Gray,

            bg_light: Color::Rgb(40, 40, 40),
            bg_dark: Color::Rgb(28, 28, 28),

            bg_grid_highlight: Color::Rgb(60, 60, 60),
            bg_num_highlight: Color::Rgb(90, 90, 90),
            bg_selected: Color::Rgb(110, 110, 110),

            bg_invalid: Color::Rgb(120, 20, 20),

            number_fixed: Color::Cyan,
            number_user: Color::White,

            note: Color::DarkGray,
        }
    }
}

const CELL_WIDTH: u16 = 7;
const CELL_HEIGHT: u16 = 4;

const NOTE_POSITIONS: [(u16, u16); 9] = [
    (1, 1), (3, 1), (5, 1),
    (1, 2), (3, 2), (5, 2),
    (1, 3), (3, 3), (5, 3),
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
    let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
        Constraint::Length(70),
        Constraint::Min(20),
    ])
    .split(frame.area());

    render_board(frame, app, chunks[0]);
    render_sidebar(frame, app, chunks[1]);
}

pub fn render_sidebar(frame: &mut Frame, app: &App, area: Rect) {
    let buf = frame.buffer_mut();

    buf.set_string(
        area.x + 1,
        area.y + 1,
        "TUI Doku",
        Style::default().fg(app.theme.number_fixed).add_modifier(Modifier::BOLD),
    );

    let elapsed = match app.end_time {
        Some(end) => end,
        None => app.start_time.elapsed(),
    };

    let timer = format!(
        "Time: {}",
        format_duration(elapsed),
    );

    buf.set_string(
        area.x + 1,
        area.y + 3,
        timer,
        Style::default().fg(app.theme.number_user),
    );

    let lines = [
        "Use arrow keys to move",
        "Press 1-9 to input numbers",
        "Press Backspace to clear",
        "Press 'q' to quit",
    ];

    for (i, line) in lines.iter().enumerate() {
        buf.set_string(
            area.x + 1,
            area.y + 5 + i as u16,
            *line,
            Style::default().fg(app.theme.number_user),
        );
    }
    let notes_text = if app.notes {
        "Notes: ON (press 'n' to toggle)"
    } else {
        "Notes: OFF (press 'n' to toggle)"
    };
    buf.set_string(area.x + 1, area.y + 10, notes_text, Style::default().fg(app.theme.note));

    if app.end_time.is_some() {
        buf.set_string(
            area.x + 1,
            area.y + 14,
            "Congratulations! You solved the puzzle!",
            Style::default().fg(app.theme.number_fixed).add_modifier(Modifier::BOLD),
        );
    }
}

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();

    let hours = secs / 3600;
    let mins = (secs % 3600) / 60;
    let secs = secs % 60;

    if hours > 0 {
        format!("{hours:02}:{mins:02}:{secs:02}")
    } else {
        format!("{mins:02}:{secs:02}")
    }
}

pub fn render_board(frame: &mut Frame, app: &App, area: Rect) {
    let visual_board = build_board(app);

    let area = frame.area();
    let buf = frame.buffer_mut();

    let board_x = area.x + 2;
    let board_y = area.y + 1;

    for row in 0..9 {
        for col in 0..9 {
            let rect = Rect {
                x: board_x + col as u16 * CELL_WIDTH,
                y: board_y + row as u16 * CELL_HEIGHT,
                width: CELL_WIDTH,
                height: CELL_HEIGHT,
            };

            render_cell(
                app,
                buf,
                rect,
                row,
                col,
                &visual_board[row][col],
            );
        }
    }

    draw_grid(app, buf, board_x, board_y);
}

fn build_board(app: &App) -> [[CellData; 9]; 9] {
    std::array::from_fn(|row| {
        std::array::from_fn(|col| {
            let cell = &app.board.cells[row][col];

            CellData {
                value: cell.value,

                notes: cell.notes,

                fixed: cell.fixed,

                is_valid: cell.is_valid,

                selected:
                    row == app.selected_row
                    && col == app.selected_col,
            }
        })
    })
}

fn render_cell(
    app: &App,
    buf: &mut Buffer,
    area: Rect,
    row: usize,
    col: usize,
    cell: &CellData,
) {
    draw_background(
        app,
        buf,
        area,
        row,
        col,
        cell
    );

    match cell.value {
        Some(value) => {
            draw_big_number(
                app,
                buf,
                area,
                value,
                cell.fixed,
            );
        }

        None => {
            draw_notes(
                app,
                buf,
                area,
                &cell.notes,
            );
        }
    }
}

fn draw_background(
    app: &App,
    buf: &mut Buffer,
    area: Rect,
    row: usize,
    col: usize,
    cell: &CellData,
) {
    let subgrid_x = col / 3;
    let subgrid_y = row / 3;

    let checker = (subgrid_x + subgrid_y) % 2 == 0;

    let bg = if cell.selected && cell.is_valid {
        app.theme.bg_selected
    } else if !cell.is_valid {
        app.theme.bg_invalid
    } else if cell.value.is_some() && cell.value == app.board.cells[app.selected_row][app.selected_col].value { // all numbers same as selected
        app.theme.bg_num_highlight
    } else if row == app.selected_row || col == app.selected_col {
        app.theme.bg_grid_highlight
    } else if checker {
        app.theme.bg_dark
    } else {
        app.theme.bg_light
    };

    for y in area.y+1..area.y + area.height {
        for x in area.x+1..area.x + area.width {
            buf[(x, y)]
                .set_char(' ')
                .set_bg(bg);
        }
    }
}

fn draw_notes(
    app: &App,
    buf: &mut Buffer,
    area: Rect,
    notes: &[bool; 9],
) {
    for i in 0..9 {
        if notes[i] {
            let (dx, dy) = NOTE_POSITIONS[i];

            buf[(area.x + dx, area.y + dy)]
                .set_char(
                    char::from_digit(
                        (i + 1) as u32,
                        10,
                    )
                    .unwrap(),
                )
                .set_fg(app.theme.note);
        }
    }
}

fn draw_big_number(
    app: &App,
    buf: &mut Buffer,
    area: Rect,
    value: u8,
    fixed: bool,
) {
    let x = area.x + area.width / 2;
    let y = area.y + area.height / 2;

    let fg = if fixed {
        app.theme.number_fixed
    } else {
        app.theme.number_user
    };

    let style = Style::default()
        .fg(fg)
        .add_modifier(Modifier::BOLD);

    buf.set_string(
        x,
        y,
        value.to_string(),
        style,
    );
}

fn draw_grid(
    app: &App,
    buf: &mut Buffer,
    board_x: u16,
    board_y: u16,
) {
    let board_width = CELL_WIDTH * 9;
    let board_height = CELL_HEIGHT * 9;

    //
    // HORIZONTAL LINES
    //
    for row in 0..=9 {
        let y = board_y + row * CELL_HEIGHT;

        let thick = row % 3 == 0;
 
        let horizontal = if thick { '═' } else { '─' };

        for x in board_x..=board_x + board_width {
            buf[(x, y)]
                .set_char(horizontal)
                .set_fg(app.theme.grid);
        }
    }

    //
    // VERTICAL LINES
    //
    for col in 0..=9 {
        let x = board_x + col * CELL_WIDTH;

        let thick = col % 3 == 0;
 
        let vertical = if thick { '║' } else { '│' };

        for y in board_y..=board_y + board_height {
            buf[(x, y)]
                .set_char(vertical)
                .set_fg(app.theme.grid);
        }
    }

    //
    // INTERSECTIONS
    //
    for row in 0..=9 {
        for col in 0..=9 {
            let x = board_x + col * CELL_WIDTH;
            let y = board_y + row * CELL_HEIGHT;

            let thick_row = row % 3 == 0;
            let thick_col = col % 3 == 0;

            let ch = match (
                row,
                col,
                thick_row,
                thick_col,
            ) {
                (0, 0, _, _) => '╔',
                (0, 9, _, _) => '╗',
                (9, 0, _, _) => '╚',
                (9, 9, _, _) => '╝',

                (0, _, _, true) => '╦',
                (9, _, _, true) => '╩',

                (_, 0, true, _) => '╠',
                (_, 9, true, _) => '╣',

                (0, _, _, _) => '╤',
                (9, _, _, _) => '╧',

                (_, 0, _, _) => '╟',
                (_, 9, _, _) => '╢',

                (_, _, true, true) => '╬',

                (_, _, true, false) => '╪',

                (_, _, false, true) => '╫',

                _ => '┼',
            };

            buf[(x, y)]
                .set_char(ch)
                .set_fg(app.theme.grid);
        }
    }
}