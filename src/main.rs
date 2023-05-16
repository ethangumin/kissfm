use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use state::App;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame, Terminal,
};

mod commands;
mod helpful_commands;
mod navigation_window;
mod state;
mod working_directory;

// Files
mod settings;

fn main() -> Result<(), Box<dyn Error>> {
    // print user conf
    settings::get_conf();
    println!("{:?}", commands::ls("./"));
    println!("{:?}", commands::ls("./ -a"));
    println!("{:?}", commands::ls("./ -a -l"));

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('j') => app.items.next(),
                KeyCode::Char('k') => app.items.previous(),
                KeyCode::Enter => {
                    let current_path = working_directory::get_working_dir();
                    if let Some(selected_file) = app.items.get_selected() {
                        let new_path = current_path + "/" + selected_file;
                        return commands::enter_file(new_path);
                    } else {
                        println!("No file/directory currently selected");
                    }
                }
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(f.size());

    let section_titles = ["Helpful Commands", "Working Directory", "Navigator Window"];

    for (idx, title) in section_titles.iter().enumerate() {
        let block = Block::default()
            .title(title.to_string())
            .borders(Borders::ALL);

        match idx {
            0 => {
                let helpful_commands_widget = helpful_commands::generate_content().block(block);
                f.render_widget(helpful_commands_widget, layout[idx]);
            }
            1 => {
                let working_directory_widget = working_directory::generate_content().block(block);
                // should be render_stateful_widget
                f.render_widget(working_directory_widget, layout[idx]);
            }
            _ => {
                let state = app.current_files();
                let nav_window_items = navigation_window::generate_content(&state);

                f.render_stateful_widget(
                    nav_window_items.block(block),
                    layout[idx],
                    &mut app.items.state,
                );
            }
        }
    }
}
