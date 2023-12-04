use ratatui::{prelude::*, widgets::*};
use std::rc::Rc;
use crate::app::{MainWindows, App};

pub fn render_main(app: &mut App, frame: &mut Frame, mid_layout: &Rc<[Rect]>) {

    for (i, w) in [MainWindows::Right, MainWindows::Left].iter().enumerate() {
        if app.sel_window == *w {
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
