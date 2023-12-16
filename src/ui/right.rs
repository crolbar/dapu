use std::{rc::Rc, path::PathBuf};
use ratatui::{prelude::*, widgets::*};
use crate::app::{App, PreviewType};

pub fn render_right(app: &App, frame: &mut Frame, mid_layout: &Rc<[Rect]>) {
    match app.preview_type {
        PreviewType::Contents => render_preview_contents(app, mid_layout, frame),
        PreviewType::README | PreviewType::TODO => render_preview_readme_todo(app, mid_layout, frame),
    }
}

fn render_preview_contents(app: &App, mid_layout: &Rc<[Rect]>, frame: &mut Frame) {
    if !app.prev.dirs.is_empty() {

        let mid_right_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)])
            .margin(1)
            .split(mid_layout[1]);

        let mut lines= vec![];

        for (i, file_path) in app.prev.dirs.iter().enumerate() {
            lines.push(
                Line::from(
                    if app.is_focused_right() && app.prev.sel_dir == i {
                        format_file_name(
                            file_path.file_name().unwrap().to_str().unwrap().to_string(),
                            &file_path
                        ).on_red()
                    } else {
                        format_file_name(
                            file_path.file_name().unwrap().to_str().unwrap().to_string(),
                            &file_path
                        )
                    }
                )
            );
        }

        let y = {
            if lines.len() as u16 > mid_right_layout[0].height {
                if app.prev.sel_dir >= lines.len() - 5 { // if there are 5 row left to the bottom stop scrolling
                     // instead of adding 5 rows to the bottom add the rows remaining to the bottom
                    (app.prev.sel_dir + (lines.len() - app.prev.sel_dir)) as u16 - mid_right_layout[0].height
                } else if app.prev.sel_dir as u16 > mid_right_layout[0].height - 6 {  // if there are 5 rows left to the bottom of the visible rows start scrolling 
                    // count of rows bellow the visible rows
                    app.prev.sel_dir as u16 + 6 - mid_right_layout[0].height          // incrementing by one for each row bellow the bottom of the visible rows
                } else {0}
            } else {0}
        };

        frame.render_widget(
            Paragraph::new(lines).scroll((y, 0)),
            mid_right_layout[0]
        )
    }
}

fn render_preview_readme_todo(app: &App, mid_layout: &Rc<[Rect]>, frame: &mut Frame) {
    let parag = 
        if app.is_focused_right() {
            Paragraph::new(app.prev.file_txt.to_string()).scroll(app.prev.scroll)
                .block(Block::default().borders(Borders::BOTTOM).border_style(Style::default().red()))
        } else {
            Paragraph::new(app.prev.file_txt.to_string()).scroll(app.prev.scroll)
        };

    frame.render_widget(
        parag,
        Layout::default().constraints([Constraint::Percentage(100)]).margin(1).split(mid_layout[1])[0]
    );
}

fn format_file_name(file_name: String, file_path: &PathBuf) -> Span<'_> {
    if file_name.contains("git") {
        format!(" {}", file_name).dark_gray()

    } else if file_name.ends_with(".nix") {
        format!(" {}", file_name).blue()

    } else if file_name.contains("src") {
        format!(" {}", file_name).green()

    } else if file_name.ends_with(".lock") {
        format!(" {}", file_name).white()

    } else if file_name.ends_with(".toml") {
        format!(" {}", file_name).white()

    } else if file_name.ends_with("LICENSE") {
        format!(" {}", file_name).yellow()

    } else if file_name.ends_with("TODO.md") {
        format!(" {}", file_name).green()

    } else if file_name.ends_with(".md") {
        format!(" {}", file_name).white()

    } else if file_name.ends_with(".sh") {
        format!(" {}", file_name).green()

    } else if file_path.is_dir() {
        format!(" {}", file_name).light_blue()

    } else {
        file_name.into()
    }

}
