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

pub struct ListStateWrapper {
    pub state: ListState,
    len: usize,
}

impl ListStateWrapper {
    pub fn new(state: ListState, len: usize) -> Self {
        Self { state, len }
    }

    pub fn next(&mut self) {
        self.state.select(get_next(self.state.selected(), self.len))
    }

    pub fn previous(&mut self) {
        self.state.select(get_prev(self.state.selected(), self.len))
    }

    pub fn reset(&mut self, len: usize) {
        if self.len != len {
            self.state = ListState::default();
            self.len = len;
        }
    }
}

pub struct State {
    pub task_state: ListStateWrapper,
    pub context_state: ListStateWrapper,
    pub tag_state: ListStateWrapper,
    pub active_list: ActiveList,
}

impl State {
    //pub fn new( handle: &'a ListHandle) -> Self {
    pub fn new(tasklen: usize, ctxlen: usize, taglen: usize) -> Self {
        //let l = todo::ListRep::new(handle);

        Self {
            task_state: ListStateWrapper::new(ListState::default(), tasklen),
            context_state: ListStateWrapper::new(ListState::default(), ctxlen),
            tag_state: ListStateWrapper::new(ListState::default(), taglen),
            active_list: ActiveList::Tasks,
        }
    }

    pub fn next(&mut self) {
        self.get_active_state().next();
    }

    pub fn previous(&mut self) {
        self.get_active_state().previous();
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

    fn get_active_state(&mut self) -> &mut ListStateWrapper {
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
