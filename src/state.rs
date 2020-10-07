use tui::{
    style::{Color, Style},
    widgets::ListState,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ActiveList {
    Tasks,
    Contexts,
    Tags,
}

impl ActiveList {
    pub fn to_string(&self) -> &str {
        match self {
            ActiveList::Tasks => "Tasks",
            ActiveList::Contexts => "Contexts",
            ActiveList::Tags => "Tags",
        }
    }
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
            self.len = len;
            if let Some(i) = self.state.selected() {
                if i >= len {
                    self.state = ListState::default();
                }
            }
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
        use ActiveList::{Contexts, Tags, Tasks};
        self.active_list = match self.active_list {
            Tasks => Contexts,
            Contexts => Tags,
            Tags => Tasks,
        }
    }

    pub fn move_left(&mut self) {
        use ActiveList::{Contexts, Tags, Tasks};
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
        self.get_state_mut(self.active_list)
    }

    pub fn get_state_mut(&mut self, list_t: ActiveList) -> &mut ListStateWrapper {
        match list_t {
            ActiveList::Tasks => &mut self.task_state,
            ActiveList::Contexts => &mut self.context_state,
            ActiveList::Tags => &mut self.tag_state,
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
