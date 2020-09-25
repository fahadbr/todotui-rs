use crate::todo;

use std::{collections::HashSet, io::Error as IOErr, path::Path};
use tui::{
    style::{Color, Style},
    widgets::ListState,
};

#[derive(Debug, Eq, PartialEq)]
pub enum ActiveList {
    Tasks,
    Contexts,
    Tags,
}

pub struct State {
    pub task_state: ListState,
    pub context_state: ListState,
    pub tag_state: ListState,
    pub list: todo::List,

    active_list: ActiveList,
}

impl State {
    pub fn new() -> Result<State, IOErr> {
        const FILENAME: &'static str = "main.todo.txt";
        let todo_dir = Path::new(env!("TODO_DIR"));
        let todo_path = todo_dir.join(FILENAME);
        let l = todo::List::new(todo_path)?;

        Ok(Self {
            task_state: ListState::default(),
            context_state: ListState::default(),
            tag_state: ListState::default(),
            active_list: ActiveList::Tasks,
            list: l,
        })
    }

    pub fn next(&mut self) {
        let len = self.get_active_list_len();
        let state = self.get_active_state();

        state.select(get_next(state.selected(), len));
    }

    pub fn previous(&mut self) {
        let len = self.get_active_list_len();
        let state = self.get_active_state();

        state.select(get_prev(state.selected(), len));
    }

    pub fn move_right(&mut self) {
        use ActiveList::*;
        self.active_list = match self.active_list {
            Tasks => Contexts,
            Contexts => Tags,
            Tags => Tasks,
        }
    }

    pub fn move_left(&mut self) {
        use ActiveList::*;
        self.active_list = match self.active_list {
            Tasks => Tags,
            Contexts => Tasks,
            Tags => Contexts,
        }
    }

    pub fn get_style(&self, active_list: ActiveList) -> Style {
        if self.active_list == active_list {
            Style::default().fg(Color::White)
        } else {
            Style::default().fg(Color::DarkGray)
        }
    }

    pub fn select(& mut self, filters: &mut HashSet<String>) {
        let i = match self.get_active_state().selected() {
            Some(i) => i,
            None => return,
        };

        let filter = match self.active_list {
            ActiveList::Contexts => self.list.contexts[i].toggle_select(),
            ActiveList::Tags => self.list.tags[i].toggle_select(),
            _ => return,
        };

        if filters.contains(&filter) {
            filters.remove(&filter);
        } else {
            filters.insert(filter);
        };
    }


    fn get_active_list_len(&self) -> usize {
        use ActiveList::*;
        match self.active_list {
            Tasks => self.list.raw_items.len(),
            Contexts => self.list.contexts.len(),
            Tags => self.list.tags.len(),
        }
    }

    fn get_active_state(&mut self) -> &mut ListState {
        use ActiveList::*;
        match self.active_list {
            Tasks => &mut self.task_state,
            Contexts => &mut self.context_state,
            Tags => &mut self.tag_state,
        }
    }
}

fn get_next(opt: Option<usize>, len: usize) -> Option<usize> {
    Some(match opt {
        Some(i) if i >= len => 0,
        Some(i) => i + 1,
        None => 0,
    })
}

fn get_prev(opt: Option<usize>, len: usize) -> Option<usize> {
    Some(match opt {
        Some(i) if i == 0 => len - 1,
        Some(i) => i - 1,
        None => 0,
    })
}
