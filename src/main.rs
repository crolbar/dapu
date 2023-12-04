mod ui;

mod utils;

mod app;
use app::App;

mod tui;
use tui::Tui;

mod update;
use update::update;

use clap::Parser;
mod clapy;
use clapy::Comms;

use std::io::Result;
use std::time::Duration;

fn main() -> Result<()> {
    let args = clapy::Comms::parse();

    match args {
        Comms { add: None, remove: None } => {
            start_tui()?;
        }
        Comms {add, remove} => {
            App::add_remove_dir(add, remove);
        }
    }
    Ok(())
}

fn start_tui() -> Result<()>{
    let mut app = App::new();
    let mut tui = Tui::enter()?;

    while !app.exit {
        tui.draw(&mut app)?;
        if crossterm::event::poll(Duration::from_secs(2))? {
            update(&mut app, &mut tui)?;
        }
    }

    tui.exit()?;
    Ok(())
}
