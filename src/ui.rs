use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

const CELL_WIDTH: usize = 5;

pub fn render(frame: &mut Frame, app: &App) {
    let lines = build_board(app);

    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .title(" Sudoku ")
            .borders(Borders::ALL),
    );

    frame.render_widget(paragraph, frame.area());
}

fn build_board(app: &App) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    for row in 0..9 {
        // Thick separator every 3 rows
        if row != 0 && row % 3 == 0 {
            lines.push(Line::from(vec![
                Span::raw("──────────────────┼───────────────────┼──────────────────"),
            ]));
        }

        let mut spans = Vec::new();

        for col in 0..9 {
            // Vertical separators
            if col != 0 {
                if col % 3 == 0 {
                    spans.push(Span::styled(
                        " │ ",
                        Style::default()
                            .fg(Color::DarkGray)
                            .add_modifier(Modifier::BOLD),
                    ));
                } else {
                    spans.push(Span::raw(" "));
                }
            }

            let cell = &app.board.cells[row][col];

            let selected =
                row == app.selected_row &&
                col == app.selected_col;

            let mut style = if cell.fixed {
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Cyan)
            };

            // Checkerboard-ish subtle background
            if (row / 3 + col / 3) % 2 == 0 {
                style = style.bg(Color::Rgb(30, 30, 30));
            } else {
                style = style.bg(Color::Rgb(20, 20, 20));
            }

            // Selected cell highlight
            if selected {
                style = style
                    .bg(Color::Blue)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD);
            }

            let text = match cell.value {
                Some(v) => format_center(v.to_string()),
                None => format_center("·".to_string()),
            };

            spans.push(
                Span::styled(text, style)
            );
        }

        lines.push(Line::from(spans));
    }

    lines
}

fn format_center(value: String) -> String {
    format!(
        "{:^width$}",
        value,
        width = CELL_WIDTH
    )
}