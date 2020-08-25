use crate::todo::{Item, List};

use std::io::Error as IOErr;
use tui::widgets::{Table, TableState};

pub struct TodoListTable {
    pub state: TableState,
    pub list: List,
}

impl TodoListTable {
    pub fn new() -> Result<TodoListTable, IOErr> {
        const PATH: &str = "/data/syncthing/todo/main.todo.txt";
        let l = List::new(String::from(PATH))?;

        Ok(Self {
            state: TableState::default(),
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

    pub fn get_item(&self) -> Option<Item> {
        match self.state.selected() {
            Some(i) => Some(Item::new(&self.list.raw_items[i])),
            None => None,
        }
    }
}
