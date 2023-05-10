use std::env;
use tui::{style::Style, text::Span, widgets::Paragraph};

fn get_working_dir() -> std::string::String {
    if let Ok(current_dir) = env::current_dir() {
        return current_dir.to_string_lossy().into_owned();
    } else {
        panic!("Failed to determine the current directory.")
    }
}

pub fn generate_content() -> Paragraph<'static> {
    let content = Span::styled(
        get_working_dir(),
        Style::default().fg(tui::style::Color::Cyan),
    );

    return Paragraph::new(content);
}
