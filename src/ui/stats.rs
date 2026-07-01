use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    app::App,
    save::{StatsSave, get_stats},
};

pub fn render(frame: &mut Frame, _app: &App) {
    let lines = match get_stats() {
        Ok(stats) => build_stats_lines(&stats),
        Err(err) => vec![
            Line::from("Statistics"),
            Line::from(""),
            Line::from(format!("Failed to load statistics: {}", err)),
        ],
    };

    let paragraph =
        Paragraph::new(lines).block(Block::default().title("Statistics").borders(Borders::ALL));

    frame.render_widget(paragraph, frame.area());
}

fn build_stats_lines(stats: &StatsSave) -> Vec<Line<'static>> {
    vec![
        Line::from(format!("Games played: {}", stats.games_played)),
        Line::from(format!(
            "Total play time: {} min",
            format_time(stats.total_time_played)
        )),
        Line::from(format!(
            "Best time: {}",
            stats
                .best_time
                .map(format_time)
                .unwrap_or_else(|| "-".to_string())
        )),
        Line::from(format!(
            "Average time: {}",
            stats
                .average_time
                .map(|t| format_time(t as u64))
                .unwrap_or_else(|| "-".to_string())
        )),
    ]
}

fn format_time(seconds: u64) -> String {
    format!("{:02}:{:02}", seconds / 60, seconds % 60)
}
