use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::Paragraph,
};

pub fn generate_content() -> Paragraph<'static> {
    let commands = vec!["q"];
    let titles = vec![":quit"];

    let content = Spans::from(vec![
        Span::styled(commands[0], Style::default().fg(Color::Cyan)),
        Span::raw(titles[0].to_string()),
        Span::raw(" "),
    ]);

    return Paragraph::new(content);
}
