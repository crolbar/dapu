mod ui;

mod app;
use app::App;

mod tui;
use tui::Tui;

mod update;
use update::update;

use std::io::Result;
use std::time::Duration;

fn main() -> Result<()> {
    let mut app = App::new();
    let mut tui = Tui::enter()?;

    while !app.exit {
        tui.draw(&mut app)?;
        if crossterm::event::poll(Duration::from_secs(2))? {
            update(&mut app, &mut tui)?;
        }
    }

    Ok(())
}
