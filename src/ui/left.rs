use ratatui::{prelude::*, widgets::*};
use std::rc::Rc;
use crate::app::App;

pub fn render_left(app: &App, frame: &mut Frame, mid_layout:  &Rc<[Rect]>) {
    let mid_left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)]).margin(1)
    .split(mid_layout[0]);

    let lines: Vec<Line> = app.dirs
        .iter()
        .enumerate()
        .flat_map(|(i, dir)| {
            let name = {
                match app.dirs
                    .iter().enumerate()
                    .find(|(i2, d)| 
                          d.file_name() == dir.file_name() && i2 != &i
                    ) 
                {
                    Some(_) => format!("{}/{}", dir.parent().unwrap().file_name().unwrap().to_str().unwrap(), dir.file_name().unwrap().to_str().unwrap()),
                    None => dir.file_name().unwrap().to_str().unwrap().to_string()
                }
            };
            if i == app.sel_dir {
                [
                    Line::from("---------".red()),
                    Line::from(name.red()),
                    Line::from("---------".red()),
                ]
            } else {
                [
                    Line::from("---------".black()),
                    Line::from(name),
                    Line::from("---------".black()),
                ]
            }
        }).collect();

    let y = {
        if lines.len() as u16 > mid_left_layout[0].height {
            if app.sel_dir >= app.dirs.len() - 2 { 
                ((app.sel_dir + (app.dirs.len() - app.sel_dir)) as u16 - mid_left_layout[0].height / 3) * 3
            } else if app.sel_dir as u16 + 3 > (mid_left_layout[0].height / 3) {  
                (app.sel_dir as u16 + 3 - mid_left_layout[0].height / 3)  * 3         
            } else {0}
        } else {0}
    };

    frame.render_widget(
        Paragraph::new(lines).scroll((y, 0)),
        mid_left_layout[0]
    );
}
