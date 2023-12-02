use ratatui::{
    Frame, widgets::{Paragraph, Block, Borders},
};
use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(
            format!("text:{} \n digit:{}", app.text, app.digit)
        )
        .block(
            Block::default().borders(Borders::ALL)
        ),


        frame.size()
    )   
}
