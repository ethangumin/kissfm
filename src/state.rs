use tui::widgets::ListState;

use crate::commands::ls;

pub struct StatefulList<T> {
    pub state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        let mut state = ListState::default();
        state.select(Some(0)); // initialize state with '../' selected

        StatefulList { state, items }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0, // default selected as '../'
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0, // default selected as '../'
        };
        self.state.select(Some(i));
    }

    pub fn get_selected(&mut self) -> Option<&T> {
        if let Some(selected_index) = self.state.selected() {
            self.items.get(selected_index)
        } else {
            None
        }
    }
}

pub struct App {
    pub items: StatefulList<String>,
    pub prev: bool,
    pub file_cont: String,
}

impl App {
    pub fn new() -> App {
        let mut a = App {
            items: StatefulList::with_items(vec![]),
            prev: false,
            file_cont: String::from(""),
        };
        a.new_cwd("./", true);
        return a;
    }

    pub fn new_cwd(&mut self, args: &str, hide_dot_files: bool) {
        let files = ls(args);
        let raw_items: Vec<&str> = files.iter().map(|s| s.as_str()).collect();
        let mut items: Vec<String>;
        if hide_dot_files {
            items = raw_items
                .iter()
                .filter(|&&e| !e.starts_with(".") || e == "./" || e == "../")
                .map(|&e| e.to_string())
                .collect();
        } else {
            items = raw_items.iter().map(|&e| e.to_string()).collect();
        }

        // remove total count when showing permissions
        if args.contains('l') {
            items = items[1..].to_vec();
        }

        // sort dirs before files, then by name
        items.sort_by(|a, b| {
            let a_last_char_slash = a.chars().last() == Some('/');
            let b_last_char_slash = b.chars().last() == Some('/');

            if a_last_char_slash && !b_last_char_slash {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });

        // ../ before ./
        items.swap(0, 1);

        self.items = StatefulList::with_items(items);
    }

    pub fn current_files(&self) -> Vec<String> {
        return self.items.items.clone();
    }
}
