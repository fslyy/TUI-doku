use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::*,
};

use crate::app::App;

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
                buf,
                rect,
                row,
                col,
                &visual_board[row][col],
            );
        }
    }

    draw_grid(buf, board_x, board_y);
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
    buf: &mut Buffer,
    area: Rect,
    row: usize,
    col: usize,
    cell: &CellData,
) {
    draw_background(
        buf,
        area,
        row,
        col,
        cell.selected,
        cell.is_valid,
    );

    match cell.value {
        Some(value) => {
            draw_big_number(
                buf,
                area,
                value,
                cell.fixed,
            );
        }

        None => {
            draw_notes(
                buf,
                area,
                &cell.notes,
            );
        }
    }
}

fn draw_background(
    buf: &mut Buffer,
    area: Rect,
    row: usize,
    col: usize,
    selected: bool,
    is_valid: bool,
) {
    let subgrid_x = col / 3;
    let subgrid_y = row / 3;

    let checker = (subgrid_x + subgrid_y) % 2 == 0;

    let bg = if selected && is_valid {
        Color::Blue
    } else if !is_valid {
        Color::Red
    } else if checker {
        Color::Rgb(28, 28, 28)
    } else {
        Color::Rgb(40, 40, 40)
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
                .set_fg(Color::DarkGray);
        }
    }
}

fn draw_big_number(
    buf: &mut Buffer,
    area: Rect,
    value: u8,
    fixed: bool,
) {
    let x = area.x + area.width / 2;
    let y = area.y + area.height / 2;

    let fg = if fixed {
        Color::Cyan
    } else {
        Color::White
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
                .set_fg(Color::Gray);
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
                .set_fg(Color::Gray);
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
                .set_fg(Color::Gray);
        }
    }
}