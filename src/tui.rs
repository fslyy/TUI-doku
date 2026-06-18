use std::io::{Stdout, stdout};

use crossterm::{
    cursor,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend as Backend;

pub struct Tui {
    pub terminal: ratatui::Terminal<Backend<Stdout>>,
}

// TUI LOGIC
impl Tui {
    pub fn new() -> color_eyre::Result<Self> {
        let terminal = ratatui::Terminal::new(Backend::new(stdout()))?;

        Ok(Self { terminal })
    }

    pub fn enter(&mut self) -> color_eyre::Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(stdout(), EnterAlternateScreen, cursor::Hide)?;
        Ok(())
    }

    pub fn exit(&mut self) -> color_eyre::Result<()> {
        if crossterm::terminal::is_raw_mode_enabled()? {
            crossterm::execute!(stdout(), LeaveAlternateScreen, cursor::Show)?;
            crossterm::terminal::disable_raw_mode()?;
        }
        Ok(())
    }

    pub fn draw<F>(&mut self, render_fn: F) -> color_eyre::Result<()>
    where
        F: FnOnce(&mut ratatui::Frame),
    {
        self.terminal.draw(render_fn)?;
        Ok(())
    }
}
