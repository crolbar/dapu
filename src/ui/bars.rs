use ratatui::{prelude::*, widgets::*};
use crate::app::{App, PreviewType};
use std::rc::Rc;
use git2::Repository;

pub fn render_bars(app: &App, frame: &mut Frame, main_layout: &Rc<[Rect]>) {
    render_bottom_bar(app, frame, main_layout);

    match Repository::open(&app.dirs[app.sel_dir]) {
        Ok(repo) => render_top_bar_git(app, frame, main_layout, repo),
        Err(_) => render_top_bar(app, frame, main_layout)
    }
}

fn render_top_bar_git(_app: &App, frame: &mut Frame, main_layout: &Rc<[Rect]>, repo: Repository) {
    let time = format!("{}", chrono::Local::now().format("%Y-%m-%d %I:%M:%S %p"));

    let mut constraints = vec![
        Constraint::Percentage(100),
        Constraint::Min(time.len() as u16 + 2),
    ];

    let mut remote_urls: Vec<(String, bool)> = vec![];
    let mut branch = String::new();

    if let Ok(head) = repo.head() {
        branch = format!(" {}", head.shorthand().unwrap().to_string());

        constraints.insert(0, Constraint::Min(branch.len() as u16));
    }


    if let Ok(remotes) = repo.remotes() {
        for i in &remotes {
            let remote = repo.find_remote(i.unwrap()).unwrap();
            let remote_url = remote.url().unwrap()
                .to_string()
                .replace("http://", "")
                .replace("https://", "");

            let remote_url =
                match remote_url.contains("github") {
                    true => format!(" {}", remote_url),
                    false => format!(" {}", remote_url)
                };


            let is_up_to_date = {
                let head = repo.head().unwrap();
                let branch = head.shorthand().unwrap();

                let local_branch = repo.find_branch(
                    &branch,
                    git2::BranchType::Local
                ).unwrap().into_reference();

                let upstream_branch = 
                    repo.find_branch(
                        &format!("{}/{}",
                             remote.name().unwrap(),
                             branch
                        ),
                        git2::BranchType::Remote)
                    .unwrap().into_reference();

                let local_commit_id = local_branch.peel_to_commit().unwrap().id();
                let remote_commit_id = upstream_branch.peel_to_commit().unwrap().id();

                local_commit_id == remote_commit_id
            };

            constraints.insert(0, Constraint::Min(3));
            constraints.insert(0, Constraint::Min(remote_url.len() as u16));
            remote_urls.insert(0, (remote_url, is_up_to_date))
        }
    }


    let top_bar_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints.clone())
        .horizontal_margin(3)
        .split(main_layout[0]);


    // branch 
    if !branch.is_empty() {
        frame.render_widget(
            Paragraph::new(branch).dark_gray()
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT)),
            top_bar_layout[(top_bar_layout.len() - 3) as usize]
        );
    }


    // remotes
    if !remote_urls.is_empty() {
        for (i, (url, is_up_to_date)) in remote_urls.iter().enumerate() {
            let i = if i % 2 == 0 {i} else {i + 1};
            frame.render_widget(
                Paragraph::new(url.to_string()).gray()
                .block(Block::default().borders(Borders::LEFT)),
                top_bar_layout[i]
            );

            let parag = 
                match *is_up_to_date {
                    true => Paragraph::new("").green(),
                    false => Paragraph::new("").red()
                };

            frame.render_widget(
                parag.block(Block::default().borders(Borders::RIGHT).gray()),
                top_bar_layout[i+1]
            );
        }
    }

    // time
    frame.render_widget(
        Paragraph::new(time)
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT)),
        top_bar_layout[(top_bar_layout.len() - 1) as usize]
    );
}



fn render_top_bar(_app: &App, frame: &mut Frame, main_layout: &Rc<[Rect]>) {
    let time = format!("{}", chrono::Local::now().format("%Y-%m-%d %I:%M:%S %p"));

    let constraints = vec![
        Constraint::Percentage(100),
        Constraint::Min(time.len() as u16 + 2),
    ];

    let top_bar_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints.clone())
        .horizontal_margin(3)
        .split(main_layout[0]);

    frame.render_widget(
        Paragraph::new(time)
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT)),
        top_bar_layout[(top_bar_layout.len() - 1) as usize]
    );
}


fn render_bottom_bar(app: &App, frame: &mut Frame, main_layout: &Rc<[Rect]>) {
    let preview_mode_paragraphs = [
        (PreviewType::Contents, "c"),
        (PreviewType::README, "r"),
        (PreviewType::TODO, "t"),
    ];
    let only_output_path_label = "o";

    let constraints = [
        Constraint::Percentage(100), // git pull output

        Constraint::Min(
            (only_output_path_label.len() + 2) as u16
        ), // only output path
                                    
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

    frame.render_widget(
        Paragraph::new(app.git_pull_out.clone()),
        bottom_bar_layout[0]
    );

    { // prvu mode
        for (i, (par_type, par_text)) in preview_mode_paragraphs.iter().enumerate() {
            let paragraph =
                match app.preview_type == *par_type {
                    true => Paragraph::new(*par_text).green(),
                    false => Paragraph::new(*par_text)
                };

            frame.render_widget(
                paragraph
                .block(Block::default().borders(Borders::RIGHT | Borders::LEFT)),
                bottom_bar_layout[i+2] // +3 because of label and fill
            )
        }
    }

    { // only path
      let parag =
          match app.only_output_path {
              true => Paragraph::new(only_output_path_label).green(),
              false => Paragraph::new(only_output_path_label).red()
          };

      frame.render_widget(
          parag.block(Block::default().borders(Borders::RIGHT | Borders::LEFT)),
          bottom_bar_layout[1]
      )
    }
}
