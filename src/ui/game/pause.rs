use ratatui::{prelude::*, widgets::{Block, Borders, Paragraph}};
use crate::app::App;

pub fn render_pause_overlay(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let buf = frame.buffer_mut();

    let overlay = Rect {
        x: area.x + area.width / 4,
        y: area.y + area.height / 3,
        width: area.width / 2,
        height: area.height / 3,
    };

    for y in overlay.y..overlay.y + overlay.height {
        for x in overlay.x..overlay.x + overlay.width {
            buf[(x, y)]
                .set_char(' ')
                .set_bg(app.theme.bg_grid_highlight);
        }
    }

    let pause = r#"
██████╗  █████╗ ██╗   ██╗███████╗███████╗██████╗
██╔══██╗██╔══██╗██║   ██║██╔════╝██╔════╝██╔══██╗
██████╔╝███████║██║   ██║███████╗█████╗  ██║  ██║
██╔═══╝ ██╔══██║██║   ██║╚════██║██╔══╝  ██║  ██║
██║     ██║  ██║╚██████╔╝███████║███████╗██████╔╝
╚═╝     ╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚══════╝╚═════╝

Press P to resume
"#;

    let popup = Paragraph::new(pause)
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(app.theme.number_fixed)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(popup, overlay);
}