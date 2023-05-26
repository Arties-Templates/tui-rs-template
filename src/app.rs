use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use tui::{backend::Backend, widgets::TableState, Terminal};

use super::{app, ui};

pub struct App<'a> {
    pub tabs: Vec<&'a str>,
    pub tab_index: usize,
    pub items: Vec<Vec<&'a str>>,
    pub state: TableState,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self {
            tabs: vec!["Main", "Secondary"],
            tab_index: 0,
            items: vec![
                vec!["Hello, World!", "Goodbye Universe"],
                vec!["Hello Earth!", "Goodby Mars"],
                vec!["Starship", "Dest: Pluto", "ETA: 7months"],
            ],
            state: TableState::default(),
        }
    }

    // Tabs
    pub fn tab_next(&mut self) {
        self.tab_index = (self.tab_index + 1) % self.tabs.len()
    }

    pub fn tab_previous(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1
        } else {
            self.tab_index = self.tabs.len() - 1
        }
    }

    // Items
    pub fn item_next(&mut self) {
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

    pub fn item_previous(&mut self) {
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

    pub fn unselect(&mut self) {
        self.state.select(None)
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
                    KeyCode::Up => app.item_previous(),
                    KeyCode::Down => app.item_next(),
                    KeyCode::Backspace => app.unselect(),
                    KeyCode::Char('z') => app.tab_previous(),
                    KeyCode::Char('x') => app.tab_next(),
                    _ => {}
                }
            }
        }
    }
}
