use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use tui::{backend::Backend, Terminal};

use super::{app, ui};

pub struct App;

impl App {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn run<B: Backend>(term: &mut Terminal<B>, app: app::App) -> std::io::Result<()> {
    loop {
        term.draw(|f| ui::render(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                #[allow(clippy::single_match)]
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    _ => {}
                }
            }
        }
    }
}
