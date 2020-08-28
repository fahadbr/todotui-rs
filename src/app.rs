use crate::todo::List;

use std::{io::Error as IOErr, path::Path};
use tui::{style::{Color, Style}, widgets::ListState};

#[derive(Debug, Eq, PartialEq)]
pub enum ActiveList {
    Tasks,
    Contexts,
    Tags,
}

pub struct App {
    pub task_state: ListState,
    pub context_state: ListState,
    pub tag_state: ListState,
    pub list: List,
    active_list: ActiveList,
}

impl App {
    pub fn new() -> Result<App, IOErr> {
        const FILENAME: &'static str = "main.todo.txt";
        let todo_dir = Path::new(env!("TODO_DIR"));
        let todo_path = todo_dir.join(FILENAME);
        let l = List::new(todo_path)?;

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
        self.active_list = match self.active_list {
            ActiveList::Tasks => ActiveList::Contexts,
            ActiveList::Contexts => ActiveList::Tags,
            ActiveList::Tags => ActiveList::Tasks,
        }
    }

    pub fn move_left(&mut self) {
        self.active_list = match self.active_list {
            ActiveList::Tasks => ActiveList::Tags,
            ActiveList::Contexts => ActiveList::Tasks,
            ActiveList::Tags => ActiveList::Contexts,
        }
    }

    pub fn get_style(&self, active_list: ActiveList) -> Style {
        if self.active_list == active_list {
            Style::default().fg(Color::White)
        } else {
            Style::default().fg(Color::DarkGray)
        }
    }

    // TODO
    //pub fn select(&mut self) {
        //match self.get_active_state().selected() {
            //Some(i) => i,
            //None => {},
        //};

    //}

    fn get_active_list_len(&self) -> usize {
        match self.active_list {
            ActiveList::Tasks => self.list.raw_items.len(),
            ActiveList::Contexts => self.list.contexts.len(),
            ActiveList::Tags => self.list.tags.len(),
        }
    }

    fn get_active_state(&mut self) -> &mut ListState {
        match self.active_list {
            ActiveList::Tasks => &mut self.task_state,
            ActiveList::Contexts => &mut self.context_state,
            ActiveList::Tags => &mut self.tag_state,
        }
    }
}

fn get_next(opt: Option<usize>, len: usize) -> Option<usize> {
    Some(match opt {
        Some(i) if i >= len => 0,
        Some(i) => i+1,
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
