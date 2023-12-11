use ratatui::{prelude::*, widgets::*};
use std::rc::Rc;
use crate::app::{CurrentWindow, App};

pub fn render_main(_app: &App, frame: &mut Frame, mid_layout: &Rc<[Rect]>) {
    for (i, _) in [CurrentWindow::Left, CurrentWindow::Right].iter().enumerate() {
        frame.render_widget(
            Block::default(),
            mid_layout[i],
        );
    }
}
