use crate::app::App;
use crate::ui::format_duration;
use ratatui::{layout::Rect, prelude::*};

pub fn render_sidebar(frame: &mut Frame, app: &App, area: Rect) {
    let buf = frame.buffer_mut();

    buf.set_string(
        area.x + 1,
        area.y + 3,
        "TUI Doku",
        Style::default()
            .fg(app.theme.number_fixed)
            .add_modifier(Modifier::BOLD),
    );

    let elapsed = app.timer.elapsed();

    let timer = format!("Time: {}", format_duration(elapsed),);

    buf.set_string(
        area.x + 1,
        area.y + 5,
        timer,
        Style::default().fg(app.theme.number_user),
    );

    let control_lines = [
        "[↑/↓/←/→] Move",
        "[1-9] input numbers",
        "[Backspace] clear cell",
        "[Q] quit and save",
        "[N] switch to note mode",
        "[P] pause timer",
        "[Esc] return to main menu",
    ];

    for (i, line) in control_lines.iter().enumerate() {
        buf.set_string(
            area.x + 1,
            area.y + 7 + i as u16,
            *line,
            Style::default().fg(app.theme.number_user),
        );
    }
    let notes_text = if app.notes_mode {
        "Notes: ON"
    } else {
        "Notes: OFF"
    };
    buf.set_string(
        area.x + 1,
        area.y + 15,
        notes_text,
        Style::default().fg(app.theme.note),
    );

    let victory_lines = [
        "Congratulations! You solved the puzzle!",
        "[Enter] Start new game",
    ];
    if app.win {
        for (i, line) in victory_lines.iter().enumerate() {
            buf.set_string(
                area.x + 1,
                area.y + 15 + i as u16,
                *line,
                Style::default()
                    .fg(app.theme.number_fixed)
                    .add_modifier(Modifier::BOLD),
            );
        }
    }
}
