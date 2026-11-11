use std::time::Duration;

use ratatui::prelude::*;

use crate::app::App;
use crate::save::stats::get_stats;

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let [_, heading_row, text_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(8),
        Constraint::Min(0),
    ])
    .areas(area);

    let [_, heading_area, _] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(74),
        Constraint::Fill(1),
    ])
    .areas(heading_row);

    const HEADING: &[&str] = &[
    "███████╗████████╗ █████╗ ████████╗██╗███████╗████████╗██╗ ██████╗███████╗",
    "██╔════╝╚══██╔══╝██╔══██╗╚══██╔══╝██║██╔════╝╚══██╔══╝██║██╔════╝██╔════╝",
    "███████╗   ██║   ███████║   ██║   ██║███████╗   ██║   ██║██║     ███████╗",
    "╚════██║   ██║   ██╔══██║   ██║   ██║╚════██║   ██║   ██║██║     ╚════██║",
    "███████║   ██║   ██║  ██║   ██║   ██║███████║   ██║   ██║╚██████╗███████║",
    "╚══════╝   ╚═╝   ╚═╝  ╚═╝   ╚═╝   ╚═╝╚══════╝   ╚═╝   ╚═╝ ╚═════╝╚══════╝",
    ];

    let stats = get_stats().ok();

    let lines = if let Some(stats) = stats {
        vec![
            format!("Games played    : {}", stats.games_played),
            format!("Play time       : {}", super::format_duration(Duration::from_secs(stats.total_time_played))),
            format!(
                "Best time       : {}",
                stats.best_time
                    .map(|t| super::format_duration(Duration::from_secs(t as u64)))
                    .unwrap_or_else(|| "-".to_string())
            ),
            format!(
                "Average time    : {}",
                stats.average_time
                    .map(|t| super::format_duration(Duration::from_secs(t as u64)))
                    .unwrap_or_else(|| "-".to_string())
            ),
            String::new(),
            "[Esc] Back".to_string(),
        ]
    } else {
        vec![
            "Failed to load statistics.".to_string(),
            String::new(),
            "[Esc] Back".to_string(),
        ]
    };

    let buf = frame.buffer_mut();

    for (i, line) in HEADING.iter().enumerate() {
        buf.set_string(
            heading_area.x + heading_area.width / 2 - 36,
            heading_area.y + i as u16,
            *line,
            Style::default()
                .fg(app.theme.number_fixed)
                .add_modifier(Modifier::BOLD),
        );
    }

    for (i, line) in lines.iter().enumerate() {
        buf.set_string(
            text_area.x + text_area.width / 2 - 15,
            text_area.y + 2 + i as u16,
            line,
            Style::default().fg(app.theme.number_user),
        );
    }
}