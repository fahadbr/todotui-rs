use crate::todo;

use std::{collections::BTreeSet, collections::HashSet, io::Error as IOErr, path::Path};
use todo::ListHandle;
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

pub struct State<'a> {
    pub task_state: ListState,
    pub context_state: ListState,
    pub tag_state: ListState,
    pub list: todo::ListRep<'a>,
    pub filtered_items: Vec<&'a str>,

    active_list: ActiveList,

    tag_filters: BTreeSet<&'a str>,
    ctx_filters: BTreeSet<&'a str>,
}

impl<'a> State<'a> {
    pub fn new( handle: &'a ListHandle) -> Self {
        let l = todo::ListRep::new(handle);

        Self {
            task_state: ListState::default(),
            context_state: ListState::default(),
            tag_state: ListState::default(),
            active_list: ActiveList::Tasks,
            tag_filters: BTreeSet::new(),
            ctx_filters: BTreeSet::new(),
            filtered_items: l.raw_items.clone(),
            list: l,
        }
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

    pub fn select(&mut self) {
        let i = match self.get_active_state().selected() {
            Some(i) => i,
            None => return,
        };

        let (filter_opt, filters) = match self.active_list {
            ActiveList::Contexts => (&mut self.list.contexts[i], &mut self.ctx_filters),
            ActiveList::Tags => (&mut self.list.tags[i], &mut self.tag_filters),
            _ => return,
        };

        filter_opt.toggle_select();
        if filter_opt.selected {
            filters.insert(filter_opt.val);
        } else {
            filters.remove(filter_opt.val);
        }

        self.refresh_filtered_list();
    }

    pub fn refresh_filtered_list(&mut self) {
        self.filtered_items = self
            .list
            .raw_items
            .iter()
            .filter(|item| {
                if !self.ctx_filters.is_empty() {
                    if None
                        == self
                            .ctx_filters
                            .iter()
                            .find(|filter| item.contains(*filter))
                    {
                        return false;
                    }
                }
                if !self.tag_filters.is_empty() {
                    if None
                        == self
                            .tag_filters
                            .iter()
                            .find(|filter| item.contains(*filter))
                    {
                        return false;
                    }
                }
                true
            })
            .map(|x| *x)
            .collect();
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
