use tui::widgets::ListState;

use crate::commands::ls;

pub struct StatefulList<T> {
    pub state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        let mut state = ListState::default();
        state.select(Some(1)); // initialize state with './' selected

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
            None => 1, // default selected as './'
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
            None => 1, // default selected as './'
        };
        self.state.select(Some(i));
    }
}

pub struct App {
    pub items: StatefulList<String>,
}

impl App {
    pub fn new() -> App {
        App {
            items: StatefulList::with_items(ls("./ -a")),
        }
    }

    pub fn current_files(&self) -> Vec<String> {
        return self.items.items.clone();
    }
}
