// iterate on enum, handle_command, execute_command as new commands are added //

use crossterm::event::{Event, KeyCode, KeyEvent};
use std::io;

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
