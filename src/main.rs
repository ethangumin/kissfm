use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use state::{App, InputMode};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    Frame, Terminal,
};

mod commands;
mod state;
mod ui;
mod utils;

// Files
mod settings;

fn main() -> Result<(), Box<dyn Error>> {
    // print user conf
    // settings::get_conf();
    // println!("{:?}", commands::ls("./"));
    // println!("{:?}", commands::ls("./ -a"));

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
    let mut hide = true;
    let mut long = false;
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('j') => app.items.next(),
                    KeyCode::Char('k') => app.items.previous(),
                    KeyCode::Char('l') => {
                        hide = false;
                        long = !long;
                        if long {
                            app.new_cwd("-l", hide)
                        } else {
                            app.new_cwd("./", hide)
                        }
                    }
                    KeyCode::Char('o') => {
                        hide = !hide;
                        long = false;
                        app.new_cwd("./", hide)
                    }
                    KeyCode::Enter => {
                        let current_path = utils::get_working_dir();
                        if let Some(selected_file) = app.items.get_selected() {
                            let new_path = current_path + "/" + selected_file;

                            if utils::is_dir(selected_file) {
                                commands::enter_dir(new_path, &mut app, "./")
                                    .expect("failed to enter directory");
                            } else {
                                commands::enter_file(new_path).expect("failed to enter file");
                            }
                        } else {
                            println!("No file/directory currently selected");
                        }
                    }
                    KeyCode::Char('%') => {
                        app.input_mode = InputMode::Editing;
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Enter => {
                        if app.input.len() != 0 {
                            let current_path = utils::get_working_dir();
                            let new_file_path = current_path + "/" + &app.input;
                            commands::create_file(new_file_path);
                        }
                    }
                    _ => {}
                },
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
                Constraint::Min(0),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    // create quick help widget
    let quick_help_widget = ui::quick_help();
    f.render_widget(quick_help_widget, layout[0]);

    // create navigation window widget
    let state = app.current_files();
    let nav_window_widget = ui::navigation_window(&state);
    f.render_stateful_widget(nav_window_widget, layout[1], &mut app.items.state);

    // create input field widget
    let input = &app.input;
    let input_field_widget = ui::input_field(input);
    f.render_widget(input_field_widget, layout[2]);
}
