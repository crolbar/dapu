pub use ratatui::prelude::*;
use crate::app::App;

mod right;
mod left;
mod bars;
mod dialog_box;

pub fn render(app: &App, frame: &mut Frame) {

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
    .split(frame.size());

    let mid_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ])
    .split(main_layout[1]);

    if !app.dirs.is_empty() {
        left::render_left(app, frame, &mid_layout);
        right::render_right(app, frame, &mid_layout);
    }

    bars::render_bars(app, frame, &main_layout);

    if app.is_focused_dialog() {
        dialog_box::render_dialog(app, frame)
    }
}
