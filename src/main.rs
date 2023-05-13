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
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame, Terminal,
};

mod commands;
mod helpful_commands;
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

    let mut state = app.current_files();

    // sort by dir, then alphabetically
    state.sort_by(|a, b| {
        let a_last_char_slash = a.chars().last() == Some('/');
        let b_last_char_slash = b.chars().last() == Some('/');

        if a_last_char_slash && !b_last_char_slash {
            std::cmp::Ordering::Less
        } else if !a_last_char_slash && b_last_char_slash {
            std::cmp::Ordering::Greater
        } else {
            a.cmp(b)
        }
    });

    let nav_window_items: Vec<ListItem> = state
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

    let nav_window_items = List::new(nav_window_items).highlight_symbol(">> ");

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
                f.render_stateful_widget(
                    nav_window_items.clone().block(block),
                    layout[idx],
                    &mut app.items.state,
                );
            }
        }
    }
}
