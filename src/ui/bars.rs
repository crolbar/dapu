use ratatui::{prelude::*, widgets::*};
use crate::app::{App, PreviewType};
use chrono::{Local, Timelike};
use std::rc::Rc;
use git2::Repository;

pub fn render_bars(app: &mut App, frame: &mut Frame, main_layout: &Rc<[Rect]>) {
    render_bottom_bar(app, frame, main_layout);
    render_top_bar(app, frame, main_layout);
}


fn render_top_bar(app: &mut App, frame: &mut Frame, main_layout: &Rc<[Rect]>) {
    let time = format!("{}:{}:{}", Local::now().hour(), chrono::Local::now().minute(), chrono::Local::now().second());

    let repo = Repository::open(&app.dirs[app.sel_dir]).unwrap();

    let branch = repo.head().unwrap();
    let branch = branch.shorthand().unwrap();
    let remote = repo.remotes().unwrap();
    let remote = remote.iter().next().unwrap().unwrap();

    let top_bar_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(100),
            Constraint::Min(remote.len() as u16),
            Constraint::Min(branch.len() as u16),
            Constraint::Min(time.len() as u16),
        ])
        .split(main_layout[0]);


    
    // branch
    frame.render_widget(
        Paragraph::new(remote),
        top_bar_layout[1]
    );

    // remotes
    frame.render_widget(
        Paragraph::new(branch),
        top_bar_layout[2]
    );

    frame.render_widget(
        Paragraph::new(time).green(),
        top_bar_layout[(top_bar_layout.len() - 1) as usize]
    );


}


fn render_bottom_bar(app: &mut App, frame: &mut Frame, main_layout: &Rc<[Rect]>) {
    let preview_mode_paragraphs = [
        (PreviewType::Contents, "c"),
        (PreviewType::README, "r"),
        (PreviewType::TODO, "t"),
    ];
    let prvu_mode_label = "prvu mode:";

    let only_output_path_label = "output path";

    let constraints = [
        Constraint::Percentage(100), //fill
                                   
        Constraint::Min(
            (only_output_path_label.len() + 2) as u16
        ), // only output path
                                    
        Constraint::Min(
            (prvu_mode_label.len()+2) as u16
        ), // prvu mode label
        Constraint::Min(
            (preview_mode_paragraphs[0].1.len()+2) as u16
        ), // prvu cont
        Constraint::Min(
            (preview_mode_paragraphs[1].1.len()+2) as u16
        ), // prvu readme 
        Constraint::Min(
            (preview_mode_paragraphs[2].1.len()+2) as u16
        ), // prvu todo 
    ];

    let bottom_bar_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .horizontal_margin(3)
        .split(main_layout[2]);

    { // prvu mode
        frame.render_widget(
            Paragraph::new(prvu_mode_label).block(Block::default().borders(Borders::LEFT)).light_green(),
            bottom_bar_layout[2]
        );

        for (i, (par_type, par_text)) in preview_mode_paragraphs.iter().enumerate() {
            let paragraph =
                match app.preview_type == *par_type {
                    true => Paragraph::new(*par_text).light_green(),
                    false => Paragraph::new(*par_text)
                };

            frame.render_widget(
                paragraph
                .block(Block::default().borders(Borders::RIGHT | Borders::LEFT)),
                bottom_bar_layout[i+3] // +3 because of label and fill
            )
        }
    }

    { // only path
      let parag =
          match app.only_output_path {
              true => Paragraph::new(only_output_path_label).light_green(),
              false => Paragraph::new(only_output_path_label).red()
          };

      frame.render_widget(
          parag.block(Block::default().borders(Borders::RIGHT | Borders::LEFT)),
          bottom_bar_layout[1]
      )
    }
}
