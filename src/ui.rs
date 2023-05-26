use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use super::{app, common::table_headers};

pub fn render<B: Backend>(term: &mut Frame<B>, app: &mut app::App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Length(10),
                Constraint::Percentage(30),
            ]
            .as_ref(),
        )
        .split(term.size());
    let rows = app.items.iter().map(|v| {
        let height = v
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = v.iter().map(|c| Cell::from(*c));
        Row::new(cells).height(height as u16)
    });
    let table = Table::new(rows)
        .header(Row::new(table_headers()))
        .block(Block::default().borders(Borders::ALL).title("Tables"))
        .highlight_style(Style::default().bg(Color::White).fg(Color::Black))
        .highlight_symbol(">) ")
        .widths(&[
            Constraint::Percentage(15),
            Constraint::Length(25),
            Constraint::Min(10),
        ]);
    term.render_stateful_widget(table, chunks[0], &mut app.state);
    let block = Block::default()
        .title("Ratui")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    term.render_widget(block, chunks[1]);

    let block2 = Block::default()
        .title("Ratui Block 2")
        .borders(Borders::ALL);
    term.render_widget(block2, chunks[2]);
}
