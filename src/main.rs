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
        Comms { add: None, remove: None } => start_tui()?,
        Comms {add, remove} => App::add_remove_dir(add, remove)
    }
    Ok(())
}

fn start_tui() -> Result<()>{
    let mut app = App::new();
    app.update_prev_dirs();

    let mut tui = Tui::enter()?;

    while !app.exit {
        tui.draw(&mut app)?;
        if poll(Duration::from_secs(2))? {
            update(&mut app, &mut tui)?;
        }
    }

    tui.exit()?;
    Ok(())
}
