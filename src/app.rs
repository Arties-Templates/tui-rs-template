use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use tui::{backend::Backend, Terminal};

use super::ui;

pub fn run<B: Backend>(term: &mut Terminal<B>) -> std::io::Result<()> {
    loop {
        term.draw(ui::render)?;

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
