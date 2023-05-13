use tui::widgets::ListState;

use crate::commands::ls;

pub struct StatefulList<T> {
    pub state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
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
