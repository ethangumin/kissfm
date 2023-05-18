use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::utils;

pub fn navigation_window(files: &Vec<String>) -> List {
    let title = Span::styled(
        utils::get_working_dir(),
        Style::default().fg(Color::LightCyan),
    );

    let nav_window_items: Vec<ListItem> = files
        .iter()
        .map(|file| {
            if let Some(last_char) = file.chars().last() {
                if last_char == '/' {
                    let prefix: String = file.chars().take(file.len() - 1).collect();

                    let prefix_as_span =
                        Span::styled(prefix, Style::default().fg(Color::LightBlue));
                    let suffix_as_span = Span::styled("/", Style::default().fg(Color::LightRed));

                    let formatted_dir = Spans::from(vec![prefix_as_span, suffix_as_span]);

                    return ListItem::new(formatted_dir);
                } else {
                    return ListItem::new(file.as_str());
                }
            }
            panic!("file/directory must have a non-empty name");
        })
        .collect();

    return List::new(nav_window_items)
        .highlight_symbol(">> ")
        .block(Block::default().title(title).borders(Borders::ALL));
}

pub fn quick_help() -> Paragraph<'static> {
    let commands = vec!["q", "j/k", "%", "d"];
    let titles = vec![":quit", ":scroll", ":new file", ":new dir"];

    let content = Spans::from(vec![
        Span::styled(commands[0], Style::default().fg(Color::Cyan)),
        Span::raw(titles[0].to_string()),
        Span::raw("  "),
        Span::styled(commands[1], Style::default().fg(Color::Cyan)),
        Span::raw(titles[1].to_string()),
        Span::raw("  "),
        Span::styled(commands[2], Style::default().fg(Color::Cyan)),
        Span::raw(titles[2].to_string()),
        Span::raw("  "),
        Span::styled(commands[3], Style::default().fg(Color::Cyan)),
        Span::raw(titles[3].to_string()),
        Span::raw("  "),
    ]);

    return Paragraph::new(content)
        .block(Block::default().title("Quick Help").borders(Borders::ALL));
}

pub fn input_field(input: &String, input_title: String) -> Paragraph {
    return Paragraph::new(String::from(input))
        .block(Block::default().borders(Borders::ALL).title(input_title));
}
