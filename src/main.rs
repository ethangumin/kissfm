use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    Terminal,
};

mod file_navigator;
mod helpful_commands;
mod working_directory;

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
    terminal
        .draw(|f| {
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

            let helpful_commands_widget = helpful_commands::widget();
            let working_directory_widget = working_directory::widget();
            let navigator_window_widget = file_navigator::widget();

            f.render_widget(helpful_commands_widget, layout[0]);
            f.render_widget(working_directory_widget, layout[1]);
            f.render_widget(navigator_window_widget, layout[2]);
        })
        .unwrap();

    helpful_commands::close_tui();

    Ok(())
}
