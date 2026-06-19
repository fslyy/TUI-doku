mod app;
mod board;
mod input;
mod save;
mod timer;
mod tui;
mod ui;

use app::App;
use tui::Tui;

use app::Screen;

// MAIN LOGIC
fn main() -> color_eyre::Result<()> {
    let mut tui = Tui::new()?;
    tui.enter()?;

    let mut app = App::new();

    while app.running {
        tui.draw(|frame| match app.screen {
            Screen::MainMenu => ui::mainmenu::render(frame, &app),
            Screen::Game => ui::game::render(frame, &app),
        })?;

        match app.screen {
            Screen::MainMenu => input::mainmenu::handle_input(&mut app),
            Screen::Game => input::game::handle_input(&mut app),
        }?
    }

    //TO-DO: persistent game safe (save board state )

    let _ = tui.exit();

    Ok(())
}
