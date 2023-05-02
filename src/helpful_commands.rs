use crossterm::event::{self, Event, KeyCode};
use tui::widgets::{Block, Borders};

pub fn widget() -> Block<'static> {
    return Block::default()
        .title("Helpful Commands")
        .borders(Borders::ALL);
}

// temporary function to close app...eventually have an enum based on keys a user presses
pub fn close_tui() {
    loop {
        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Char('q') => {
                    break;
                }
                _ => {}
            }
        }
    }
}
