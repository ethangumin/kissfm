// iterate on enum/functions in this file as new helpful commands are added

use crossterm::event::{Event, KeyCode, KeyEvent};
use std::io;
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::Paragraph,
};

enum HelpfulCommands {
    Quit,
}

pub fn handle_command() -> io::Result<()> {
    loop {
        let event = crossterm::event::read()?;
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => {
                let command = match (code, modifiers) {
                    (KeyCode::Char('q'), _) => HelpfulCommands::Quit,
                    _ => continue,
                };
                return execute_command(command);
            }
            _ => continue,
        }
    }
}

fn execute_command(command: HelpfulCommands) -> io::Result<()> {
    match command {
        HelpfulCommands::Quit => return Ok(()),
    }
}

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
