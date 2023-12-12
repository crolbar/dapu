use ratatui::{prelude::*, widgets::*};
use crate::app::App;


pub fn render_dialog(app: &App, frame: &mut Frame) {
    let main_rect = create_rect(frame);

    frame.render_widget(Clear, main_rect);
    frame.render_widget(
        Block::default().borders(Borders::ALL).border_style(Style::default().red()),
        main_rect
    );

    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .margin(1)
    .split(main_rect);


    render_left(app, frame, main_layout[0]);
    render_right(app, frame, main_layout[1]);

}

fn render_left(app: &App, frame: &mut Frame, left_rect: Rect) {
    let mut constraints = vec![];

    for _ in 0..app.dialogbox.dirs.len() {
        constraints.push(Constraint::Min(1));
    }
    constraints.push(Constraint::Percentage(100));

    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .margin(1)
        .split(left_rect);

    for (i, f) in app.dialogbox.dirs.iter().enumerate() {
        if i == app.dialogbox.sel_dir {
            frame.render_widget(
                Paragraph::new(f.file_name().unwrap().to_str().unwrap()).on_red(),
                left_layout[i]
            );
        } else {
            frame.render_widget(
                Paragraph::new(f.file_name().unwrap().to_str().unwrap()),
                left_layout[i]
            );
        }
    }
}

fn render_right(app: &App, frame: &mut Frame, right_rect: Rect) {
    let mut constraints = vec![];

    for _ in 0..app.dialogbox.preview_dirs.len() {
        constraints.push(Constraint::Min(1));
    }
    constraints.push(Constraint::Percentage(100));

    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .margin(1)
        .split(right_rect);

    for (i, f) in app.dialogbox.preview_dirs.iter().enumerate() {
        frame.render_widget(
            Paragraph::new(f.file_name().unwrap().to_str().unwrap()),
            left_layout[i]
        );
    }
}

fn create_rect(frame: &mut Frame) -> Rect {
    let hor_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(70),
            Constraint::Percentage(15),
        ])
        .split(frame.size());

    let ver_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(hor_split[1]);

    ver_split[1]
}
