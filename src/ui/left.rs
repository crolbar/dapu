use ratatui::{prelude::*, widgets::*};
use std::rc::Rc;
use crate::app::App;

pub fn render_left(app: &mut App, frame: &mut Frame, mid_layout:  &Rc<[Rect]>) {

    let mut constraints = vec![];
    for _ in &app.dirs  {
        constraints.push(Constraint::Min(2))
    }
    constraints.push(Constraint::Percentage(100));

    let mid_left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints).margin(1)
    .split(mid_layout[0]);

    for (i, dir) in app.dirs.iter().enumerate() {
        if i == app.sel_dir {
            frame.render_widget(
                Paragraph::new(dir.file_name().unwrap().to_str().unwrap()).on_red(),
                mid_left_layout[i]
            );
        } else {
            frame.render_widget(
                Paragraph::new(dir.file_name().unwrap().to_str().unwrap()),
                mid_left_layout[i]
            );
        }
    }



}
