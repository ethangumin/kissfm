use tui::widgets::{Block, Borders};

pub fn widget() -> Block<'static> {
    return Block::default()
        .title("File Navigator")
        .borders(Borders::ALL);
}
