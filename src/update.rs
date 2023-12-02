use crossterm::event::KeyCode;
use crate::{app::App, tui::Tui};
use std::io::Result;


pub fn update(app: &mut App, term: &mut Tui) -> Result<()> {
  if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
      match key.code {

        KeyCode::Char('q') => {app.exit = true; term.exit()?},

        KeyCode::Char('k') => app.digit += 1,
        KeyCode::Char('j') => app.digit -= 1,



        _ => (),
    }
  }


  Ok(())
}
