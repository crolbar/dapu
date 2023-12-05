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

    let constraints = [
        Constraint::Percentage(95), //fill
        Constraint::Min(11), // prvu mode label
        Constraint::Min(3), // prvu cont
        Constraint::Min(3), // prvu readme
        Constraint::Min(3), //prvu todo
        Constraint::Min(2), // fill
    ];
    let constrains_last_index = &constraints.len() - 1;

    let top_bar_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(main_layout[2]);



    frame.render_widget(
        Paragraph::new("prvu mode:").block(Block::default().borders(Borders::LEFT)).light_green(),
        top_bar_layout[1]
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
            top_bar_layout[i+2] // +2 because of label and fill
        )
    }

    // fill gaps
    frame.render_widget(Block::new().borders(Borders::TOP), top_bar_layout[0]);
    frame.render_widget(Block::new().borders(Borders::TOP), top_bar_layout[constrains_last_index]);

}
