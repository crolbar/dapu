use ratatui::{prelude::*, widgets::*};
use crate::app::{App, FloatWindows, CurrentWindow};

pub fn render_float(app: &mut App, frame: &mut Frame) {
    if app.sel_window != CurrentWindow::Left && app.sel_window != CurrentWindow::Right {

        let popup_pos = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                         Constraint::Percentage(25),
                         Constraint::Percentage(50),
                         Constraint::Percentage(25),
                    ])
                .split(frame.size())[1]
            )
        [1];

        match app.sel_window {
            CurrentWindow::Float(FloatWindows::AddFolder) => render_add_remove_folder(app, frame, popup_pos),
            CurrentWindow::Float(FloatWindows::ChangeEditor) => render_change_editor(app, frame, popup_pos),
            CurrentWindow::Float(FloatWindows::EditCustomComm) => render_change_comm(app, frame, popup_pos),
            _ => unreachable!()
        }
    }
}

fn render_change_comm(_app: &mut App, frame: &mut Frame, popup_pos: Rect) {
    frame.render_widget(Clear, popup_pos);

    frame.render_widget(
        Paragraph::new("comm")
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).magenta()),

        popup_pos 
   );
}

fn render_change_editor(_app: &mut App, frame: &mut Frame, popup_pos: Rect) {
    frame.render_widget(Clear, popup_pos);

    frame.render_widget(
        Paragraph::new("editor")
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).magenta()),

        popup_pos 
   );

}

fn render_add_remove_folder(_app: &mut App, frame: &mut Frame, popup_pos: Rect) {
    frame.render_widget(Clear, popup_pos);

    frame.render_widget(
        Paragraph::new("folder")
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).magenta()),

        popup_pos 
   );
}
