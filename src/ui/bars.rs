pub use ratatui::{prelude::*, widgets::*};
use crate::app::{App, PreviewType};
use std::rc::Rc;

pub fn render_bars(app: &mut App, frame: &mut Frame, main_layout: &Rc<[Rect]>) {
    render_bottom_bar(app, frame, main_layout);
    render_top_bar(app, frame, main_layout);
}


fn render_top_bar(_app: &mut App, frame: &mut Frame, main_layout: &Rc<[Rect]>) {
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Status Bar"),
        main_layout[0],
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
        Constraint::Percentage(95), //fill
                                   
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
           
        Constraint::Min(2), // fill
    ];

    let bottom_bar_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
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


    // fill gaps
    frame.render_widget(Block::new().borders(Borders::TOP), bottom_bar_layout[0]);
    frame.render_widget(Block::new().borders(Borders::TOP), bottom_bar_layout[constraints.len() - 1]);

}
