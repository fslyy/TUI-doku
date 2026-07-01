use ratatui::prelude::*;

use crate::{app::App, save::load_game_state};

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let [_, logo_row, text_area] = Layout::vertical([
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Min(0),
    ])
    .areas(area);

    let [_, logo_area, _] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(73),
        Constraint::Fill(1),
    ])
    .areas(logo_row);

    const LOGO: &[&str] = &[
        "╔══╤══╤══╗  ████████╗██╗   ██╗██╗      ██████╗  ██████╗ ██╗  ██╗██╗   ██╗",
        "╟──┼──┼──╢  ╚══██╔══╝██║   ██║██║      ██╔══██╗██╔═══██╗██║ ██╔╝██║   ██║",
        "╟──┼──┼──╠═╗   ██║   ██║   ██║██║█████╗██║  ██║██║   ██║█████╔╝ ██║   ██║",
        "╚═╦╧═╤╧═╤╝─╢   ██║   ██║   ██║██║╚════╝██║  ██║██║   ██║██╔═██╗ ██║   ██║",
        "  ╟──┼──┼──╢   ██║   ╚██████╔╝██║      ██████╔╝╚██████╔╝██║  ██╗╚██████╔╝",
        "  ╚══╧══╧══╝   ╚═╝    ╚═════╝ ╚═╝      ╚═════╝  ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ",
    ];

    let mut prompt = ["[N] New Game", "[S] Stats", "[Q] Quit", ""];

    if load_game_state().is_ok() {
        prompt = ["[C] Continue", "[N] New Game", "[S] Stats", "[Q] Quit"];
    }

    let buf = frame.buffer_mut();

    for (i, line) in LOGO.iter().enumerate() {
        buf.set_string(
            logo_area.x + logo_area.width / 2 - 37,
            logo_area.y + i as u16,
            *line,
            Style::default()
                .fg(app.theme.number_fixed)
                .add_modifier(Modifier::BOLD),
        );
    }

    for (i, line) in prompt.iter().enumerate() {
        buf.set_string(
            text_area.x + text_area.width / 2 - 10,
            text_area.y + 2 + i as u16,
            *line,
            Style::default().fg(app.theme.number_user),
        );
    }
}
