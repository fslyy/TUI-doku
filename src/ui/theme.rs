use ratatui::prelude::*;

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