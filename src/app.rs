use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use tui::{backend::Backend, widgets::TableState, Terminal};

use super::{app, ui};

pub struct App<'a> {
    pub items: Vec<Vec<&'a str>>,
    pub state: TableState,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self {
            items: vec![
                vec!["Hello, World!", "Goodbye Universe"],
                vec!["Hello Earth!", "Goodby Mars"],
            ],
            state: TableState::default(),
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.state.select(Some(i))
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.state.select(Some(i));
    }
}

pub fn run<B: Backend>(term: &mut Terminal<B>, mut app: app::App) -> std::io::Result<()> {
    loop {
        term.draw(|f| ui::render(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                #[allow(clippy::single_match)]
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Up => app.next(),
                    KeyCode::Down => app.previous(),
                    _ => {}
                }
            }
        }
    }
}
