use crossterm::{
    terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen},
    execute, event::{EnableMouseCapture, DisableMouseCapture},
};
use ratatui::{Terminal, prelude::CrosstermBackend};
use std::io::{Stderr, Result, stderr};
use crate::app::App;
use crate::ui::render;

pub struct Tui {
    pub term: Terminal<CrosstermBackend<Stderr>>
}

impl Tui {
   pub fn enter() -> Result<Self> {
       enable_raw_mode()?;
       execute!(stderr(), EnterAlternateScreen, EnableMouseCapture)?;
       let mut term = Terminal::new(CrosstermBackend::new(stderr()))?;
       term.hide_cursor()?;
       term.clear()?;

       Ok(Tui{term})
   }

   pub fn draw(&mut self, app: &mut App) -> Result<()> {
       self.term.draw(|frame| render(app, frame))?;
       Ok(())
   }

   pub fn exit(&mut self) -> Result<()> {
       execute!(stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
       disable_raw_mode()?;
       self.term.show_cursor()?;
       Ok(())
   }
}
