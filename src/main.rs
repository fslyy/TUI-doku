mod ui;
mod app;
mod tui;
mod board;
mod Input;
mod timer;

use app::App;
use tui::Tui;

use app::Screen;

// MAIN LOGIC
fn main() -> color_eyre::Result<()> {
    let mut tui = Tui::new()?;
    tui.enter()?;

    let mut app = App::new();
    
    while app.running {
        tui.draw(|frame| {
            match app.screen {
                Screen::MainMenu => ui::mainmenu::render(frame, &mut app),
                Screen::Game => ui::game::render(frame, &mut app),
                _ => {}
            }
        })?;

        match app.screen {
            Screen::MainMenu => Input::mainmenu::handle_input(&mut app),
            Screen::Game => Input::game::handle_input(&mut app),
            _ => Ok(()),
        }?

    }

    tui.exit();

    Ok(())
}
