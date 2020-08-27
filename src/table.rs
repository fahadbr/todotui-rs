use crate::todo::{ParsedItem, List};

use std::io::Error as IOErr;
use tui::widgets::{ListState};

pub struct StatefulTodoList {
    pub state: ListState,
    pub list: List,
}

impl StatefulTodoList {
    pub fn new() -> Result<StatefulTodoList, IOErr> {
        const PATH: &str = "/data/syncthing/todo/main.todo.txt";
        let l = List::new(String::from(PATH))?;

        Ok(Self {
            state: ListState::default(),
            list: l,
        })
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.list.raw_items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.list.raw_items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn get_item(&self) -> Option<ParsedItem> {
        match self.state.selected() {
            Some(i) => Some(ParsedItem::new(&self.list.raw_items[i])),
            None => None,
        }
    }
}
