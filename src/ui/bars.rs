use std::rc::Rc;

pub use ratatui::{prelude::*, widgets::*};
use crate::app::App;

pub fn render_bars(_app: &mut App, frame: &mut Frame, main_layout: &Rc<[Rect]>) {
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Title Bar"),
        main_layout[0],
    );
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Status Bar"),
        main_layout[2],
    );
}
