use tui::widgets::{Block, Borders};

pub fn widget() -> Block<'static> {
    return Block::default()
        .title("Working Directory")
        .borders(Borders::ALL);
}
