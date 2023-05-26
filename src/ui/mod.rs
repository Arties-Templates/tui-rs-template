mod macros;

use tui::{
  backend::Backend,
  layout::{Alignment, Constraint, Direction, Layout},
  style::{Color, Style},
  text::{Span, Spans, Text},
  widgets::{Block, Borders, Cell, Paragraph, Row, Table, Tabs, Wrap},
  Frame,
};

use super::{app, common::table_headers};
use super::ui::macros::*;

pub fn render<B: Backend>(term: &mut Frame<B>, app: &mut app::App) {
  let chunks = Layout::default()
      .direction(Direction::Vertical)
      .margin(2)
      .constraints(
          [
              Constraint::Length(10),
              Constraint::Percentage(50),
              Constraint::Length(30),
          ]
          .as_ref(),
      )
      .split(term.size());

  let titles = app
      .tabs
      .iter()
      .map(|t| {
          let (first, last) = t.split_at(1);
          Spans::from(vec![
              Span::styled(first, Style::default().fg(Color::White)),
              Span::styled(last, Style::default().fg(Color::White)),
          ])
      })
      .collect::<Vec<Spans>>();

  render!(
      term,
      Tabs::new(titles)
          .block(Block::default().borders(Borders::ALL))
          .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
          .divider("||")
          .style(Style::default().bg(Color::Black))
          .select(app.tab_index),
      chunks[0]
  );

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

  render_stateful!(
      term,
      Table::new(rows)
          .header(Row::new(table_headers()))
          .block(Block::default().borders(Borders::ALL).title("Tables"))
          .highlight_style(Style::default().bg(Color::White).fg(Color::Black))
          .highlight_symbol(">) ")
          .widths(&[
              Constraint::Percentage(15),
              Constraint::Length(25),
              Constraint::Min(10),
          ]),
      chunks[1],
      &mut app.state
  );

  if app.tab_index == 1 {
      render!(
          term,
          Block::default()
              .title("Ratui")
              .title_alignment(Alignment::Center)
              .borders(Borders::ALL),
          chunks[2]
      );
  } else {
      render!(
          term,
          Paragraph::new(Text::styled(
              "You should move to the Secondary Tab!",
              Style::default().fg(Color::Cyan)
          ))
          .alignment(Alignment::Left)
          .wrap(Wrap { trim: true }),
          chunks[2]
      )
  }
}