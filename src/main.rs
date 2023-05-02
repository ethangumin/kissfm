use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Terminal,
};

mod helpful_commands;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create ui
    let res = ui(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn ui<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    terminal.draw(|f| {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(12),
                    Constraint::Percentage(10),
                    Constraint::Percentage(78),
                ]
                .as_ref(),
            )
            .split(f.size());

        let section_titles = ["Helpful Commands", "Working Directory", "Navigator Window"];

        for (idx, title) in section_titles.iter().enumerate() {
            let block = Block::default()
                .title(title.to_string())
                .borders(Borders::ALL);
            f.render_widget(block, layout[idx]);
        }
    })?;

    helpful_commands::close_tui();

    Ok(())
}
