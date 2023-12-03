use ratatui::{prelude::*, widgets::*};
use std::rc::Rc;
use crate::app::App;

pub fn render_main(app: &mut App, frame: &mut Frame, mid_layout: &Rc<[Rect]>) {

    for i in 0..2 {
        if app.sel_window == i {
            frame.render_widget(
                Block::default().borders(Borders::ALL).border_style(Style::default().red()),
                mid_layout[i],
            );
        } else {
            frame.render_widget(
                Block::default().borders(Borders::ALL),
                mid_layout[i],
            );
        }

    }
}
