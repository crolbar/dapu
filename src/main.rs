mod ui;
mod utils;
mod app;
mod tui;
mod update;
mod clapy;

use app::App;
use tui::Tui;
use update::update;
use clap::Parser;
use clapy::Comms;
use std::{io::Result, time::Duration};
use crossterm::event::poll;

fn main() -> Result<()> {
    match clapy::Comms::parse() {
        Comms { add: None, remove: None, only_path } => start_tui(only_path)?,
        Comms { add, remove, .. } => App::add_remove_dir(add, remove)
    }
    Ok(())
}

fn start_tui(only_path: bool) -> Result<()>{
    let mut app = App::new(only_path);
    let mut tui = Tui::enter()?;

    app.update_right_pane();

    while !app.exit {
        tui.draw(&mut app)?;
        if poll(Duration::from_secs(2))? {
            update(&mut app, &mut tui)?;
        }
    }

    tui.exit()?;
    Ok(())
}
