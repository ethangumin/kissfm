use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{List, ListItem},
};

pub fn generate_content(files: &Vec<String>) -> List {
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

    return List::new(nav_window_items).highlight_symbol(">> ");
}
