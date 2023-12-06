use std::{rc::Rc, path::PathBuf};
use ratatui::{prelude::*, widgets::*};
use crate::{app::{App, PreviewType, CurrentWindow}, utils::exit_with_err_msg};

pub fn render_right(app: &mut App, frame: &mut Frame, mid_layout: &Rc<[Rect]>) {
    let app_dirs = app.dirs.clone();
    let sel_dir_path = app_dirs[app.sel_dir].to_str().unwrap();

    match app.preview_type {
        PreviewType::Contents => render_preview_contents(app, sel_dir_path, mid_layout, frame),
        PreviewType::README => render_preview_readme(sel_dir_path, mid_layout, frame),
        PreviewType::TODO => render_preview_todo(sel_dir_path, mid_layout, frame),
    }
}

fn render_preview_contents(app: &mut App, sel_dir_path: &str, mid_layout: &Rc<[Rect]>, frame: &mut Frame) {
    if let Ok(read_dir) = std::fs::read_dir(sel_dir_path) {

        app.preview_conts_dirs = 
            read_dir.map(|f| 
                f.unwrap_or_else(|_| {
                    exit_with_err_msg("No permissions to read file in directory or file dosnt exist");
                    unreachable!()
                }).path()
            ).collect();

        let mut constraints = vec![];
        for _ in 0..app.preview_conts_dirs.len() {
            constraints.push(Constraint::Min(1))
        }
        // fill the empty space
        constraints.push(Constraint::Percentage(100));

        let mid_right_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .margin(1)
            .split(mid_layout[1]);


        for (i, file_path) in app.preview_conts_dirs.iter().enumerate() {
            let mut file_paragrapth = 
                format_file_p(
                    file_path.file_name().unwrap().to_str().unwrap().to_string(),
                    &file_path
                );
            if app.sel_window == CurrentWindow::Right && app.sel_prev_conts_dir == i {
                file_paragrapth = file_paragrapth.on_red();
            }

            frame.render_widget(
                file_paragrapth,
                mid_right_layout[i]
            )
        }
    }

}

fn render_preview_readme(sel_dir_path: &str, mid_layout: &Rc<[Rect]>, frame: &mut Frame) {
    if let Ok(read_dir) = &mut std::fs::read_dir(sel_dir_path) {
        if let Some(readme) = read_dir.find(|f| f.as_ref().unwrap().file_name().to_str().unwrap().contains("README")) {
            frame.render_widget(
                Paragraph::new(std::fs::read_to_string(readme.unwrap().path()).unwrap()),
                Layout::default().constraints([Constraint::Percentage(100)]).margin(1).split(mid_layout[1])[0]
            )
        }

    }
}

fn render_preview_todo(sel_dir_path: &str, mid_layout: &Rc<[Rect]>, frame: &mut Frame) {
    if let Ok(read_dir) = &mut std::fs::read_dir(sel_dir_path) {
        if let Some(todo) = read_dir.find(|f| f.as_ref().unwrap().file_name().to_str().unwrap().contains("TODO")) {
            frame.render_widget(
                Paragraph::new(std::fs::read_to_string(todo.unwrap().path()).unwrap()),
                Layout::default().constraints([Constraint::Percentage(100)]).margin(1).split(mid_layout[1])[0]
            )
        }
    }

}



fn format_file_p(file_name: String, file_path: &PathBuf) -> Paragraph {
    if file_name.contains("git") {
        Paragraph::new(format!(" {}", file_name)).dark_gray()
    } else if file_name.ends_with(".nix") {
        Paragraph::new(format!(" {}", file_name)).blue()
    } else if file_name.contains("src") {
        Paragraph::new(format!(" {}", file_name)).green()
    } else if file_name.ends_with(".lock") {
        Paragraph::new(format!(" {}", file_name)).white()
    } else if file_name.ends_with(".toml") {
        Paragraph::new(format!(" {}", file_name)).white()
    } else if file_name.ends_with("LICENSE") {
        Paragraph::new(format!(" {}", file_name)).yellow()
    } else if file_name.ends_with("TODO.md") {
        Paragraph::new(format!(" {}", file_name)).green()
    } else if file_name.ends_with(".md") {
        Paragraph::new(format!(" {}", file_name)).white()
    } else if file_path.is_dir() {
        Paragraph::new(format!(" {}", file_name)).light_blue()
    } else {
        Paragraph::new(file_name)
    }
}
