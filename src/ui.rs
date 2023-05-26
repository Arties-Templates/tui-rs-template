use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame,
};

use super::app;

pub fn render<B: Backend>(term: &mut Frame<B>, _app: &app::App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(term.size());

    let block = Block::default().title("Ratui").borders(Borders::ALL);
    term.render_widget(block, chunks[1]);

    let block2 = Block::default()
        .title("Ratui Block 2")
        .borders(Borders::ALL);
    term.render_widget(block2, chunks[2]);
}
