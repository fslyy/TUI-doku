mod ui;
mod app;
mod tui;
mod input;
mod board;

use app::App;
use tui::Tui;

// MAIN LOGIC
fn main() -> color_eyre::Result<()> {
    let mut tui = Tui::new()?;
    tui.enter()?;

    let mut app = App::new();
    
    while app.running {
        tui.draw(|frame| {
            ui::render(frame, &app);
        })?;

        input::handle_input(&mut app)?;
    }

    tui.exit();

    Ok(())
}
