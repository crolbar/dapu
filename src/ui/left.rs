use ratatui::{prelude::*, widgets::*, symbols::border};
use std::rc::Rc;
use crate::app::App;

pub fn render_left(app: &App, frame: &mut Frame, mid_layout:  &Rc<[Rect]>) {

    let mut constraints = vec![];
    for _ in 0..app.dirs.len()  {
        constraints.push(Constraint::Min(3))
    }
    constraints.push(Constraint::Percentage(100));

    let mid_left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints).margin(1)
    .split(mid_layout[0]);

    for (i, dir) in app.dirs.iter().enumerate() {
        if i == app.sel_dir {
            frame.render_widget(
                Paragraph::new(dir.file_name().unwrap().to_str().unwrap())
                .block(Block::default().borders(Borders::ALL).border_set(border::ROUNDED).border_style(Style::default().red())),
                mid_left_layout[i]
            );
        } else {
            frame.render_widget(
                Paragraph::new(dir.file_name().unwrap().to_str().unwrap())
                .block(Block::default().borders(Borders::ALL).border_set(border::ROUNDED).border_style(Style::default().black())),
                mid_left_layout[i]
            );
        }
    }



}
