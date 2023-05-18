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
    widgets::{Block, Borders, Clear, Paragraph},
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
    // println!("{}", commands::prev_file(String::from("./Cargo.toml")));

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
                    KeyCode::Char('j') => {
                        app.prev = false;
                        app.items.next()
                    }
                    KeyCode::Char('k') => {
                        app.prev = false;
                        app.items.previous()
                    }
                    KeyCode::Char('p') => {
                        if let Some(selected_file) = app.items.get_selected() {
                            let current_path = utils::get_working_dir();
                            let new_path = current_path + "/" + selected_file;
                            let preview: String;
                            if selected_file.ends_with("/") {
                                preview = commands::ls(&new_path).join("\n");
                            } else {
                                preview = commands::prev_file(new_path);
                            }
                            app.prev = !app.prev;
                            app.file_cont = preview;
                        }
                    }
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
                        app.input_field_title = String::from("Enter Filename")
                    }
                    KeyCode::Char('d') => {
                        app.input_mode = InputMode::Editing;
                        app.input_field_title = String::from("Enter Directory Name")
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
                        commands::restore_input_field(&mut app);
                    }
                    KeyCode::Enter => {
                        if app.input.len() != 0 {
                            let current_path = utils::get_working_dir();
                            let new_path = current_path + "/" + &app.input;
                            let title = app.input_field_title.clone();

                            if title == String::from("Enter Filename") {
                                commands::create_file(new_path);
                            } else {
                                commands::create_dir(new_path, &mut app, hide.clone());
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let input_mode = &app.input_mode;
    let mut layout_constraints = [
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(3),
    ]
    .to_vec();

    match input_mode {
        // hide input bar if in normal mode
        InputMode::Normal => {
            layout_constraints.pop();
        }
        _ => {}
    }

    let size = f.size();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(layout_constraints.as_ref())
        .split(f.size());

    // create quick help widget
    let quick_help_widget = ui::quick_help();
    f.render_widget(quick_help_widget, layout[0]);

    // create navigation window widget
    let state = app.current_files();
    let nav_window_widget = ui::navigation_window(&state);
    f.render_stateful_widget(nav_window_widget, layout[1], &mut app.items.state);

    // create input field widget
    match input_mode {
        InputMode::Editing => {
            let input = &app.input;
            let input_title = app.input_field_title.clone();
            let input_field_widget = ui::input_field(input, input_title);
            f.render_widget(input_field_widget, layout[2]);
        }
        _ => {}
    }

    if app.prev {
        let block = Paragraph::new(app.file_cont.clone())
            .block(Block::default().borders(Borders::ALL).title(" Preview "))
            .alignment(tui::layout::Alignment::Left);
        let area = centered_rect(80, 80, size);
        f.render_widget(Clear, area);
        f.render_widget(block, area);
    }
}

// preview window
fn centered_rect(percent_x: u16, percent_y: u16, r: tui::layout::Rect) -> tui::layout::Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
