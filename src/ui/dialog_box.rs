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
    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .margin(1)
        .split(left_rect);

    let lines: Vec<Line> = app
        .dialogbox.dirs
        .iter().enumerate().map(|(i, d)|{
            let span = 
                if i == app.dialogbox.sel_dir {
                    d.file_name().unwrap().to_str().unwrap().on_red()
                } else {
                    Span::from(d.file_name().unwrap().to_str().unwrap())
                };

            if app.dirs.contains(&d.canonicalize().unwrap()) {
                return Line::from(vec![span, " ".green()])
            }

            Line::from(span)
    }).collect();

    let y = {
        if lines.len() as u16 > left_layout[0].height {
            if app.dialogbox.sel_dir >= app.dialogbox.dirs.len() - 5 { 
                (app.dialogbox.sel_dir + (lines.len() - app.dialogbox.sel_dir)) as u16 - left_layout[0].height
            } else if app.dialogbox.sel_dir as u16 > left_layout[0].height - 6 {  
                app.dialogbox.sel_dir as u16 + 6 - left_layout[0].height
            } else {0}
        } else {0}
    };


    frame.render_widget(
        Paragraph::new(lines).scroll((y, 0)),
        left_layout[0]
    );
}

fn render_right(app: &App, frame: &mut Frame, right_rect: Rect) {
    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .margin(1)
        .split(right_rect);

    frame.render_widget(
        Paragraph::new(
            app.dialogbox.preview_dirs
                .iter().map(|d| 
                    Line::from(d.file_name().unwrap().to_str().unwrap())
                ).collect::<Vec<Line>>()
        ),
        left_layout[0]
    )
}

fn create_rect(frame: &mut Frame) -> Rect {
    let hor_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(frame.size());

    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(70),
            Constraint::Percentage(15),
        ])
        .split(hor_split[1])
    [1]
}
