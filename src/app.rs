use crate::{
    event::{Event, Handler as EventHandler},
    runner::Action,
};
use crate::{filters::Filters, todo::ParsedLine};

use termion::event::Key;
use tui::{
    backend::Backend,
    layout::{Constraint::Percentage, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
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

pub struct BlockState {
    pub pos: ListState,
    len: usize,
}

impl BlockState {
    pub fn new(state: ListState, len: usize) -> Self {
        Self { pos: state, len }
    }

    pub fn next(&mut self) {
        self.pos.select(Some(match self.pos.selected() {
            Some(i) if i < self.len => i + 1,
            _ => 0,
        }))
    }

    pub fn previous(&mut self) {
        self.pos.select(Some(match self.pos.selected() {
            Some(i) if i == 0 => self.len - 1,
            Some(i) => i - 1,
            None => 0,
        }))
    }

    pub fn reset(&mut self, len: usize) {
        if self.len != len {
            self.len = len;
            if let Some(i) = self.pos.selected() {
                if i >= len {
                    self.pos = ListState::default();
                }
            }
        }
    }
}

pub struct State {
    pub tasks: BlockState,
    pub contexts: BlockState,
    pub tags: BlockState,
    pub active_list: ActiveList,
}

impl State {
    pub fn new(tasklen: usize, ctxlen: usize, taglen: usize) -> Self {
        Self {
            tasks: BlockState::new(ListState::default(), tasklen),
            contexts: BlockState::new(ListState::default(), ctxlen),
            tags: BlockState::new(ListState::default(), taglen),
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

    fn get_active_state(&mut self) -> &mut BlockState {
        self.get_state_mut(self.active_list)
    }

    pub fn get_state_mut(&mut self, list_t: ActiveList) -> &mut BlockState {
        match list_t {
            ActiveList::Tasks => &mut self.tasks,
            ActiveList::Contexts => &mut self.contexts,
            ActiveList::Tags => &mut self.tags,
        }
    }
}

pub struct MainView<'a> {
    //pub state: RefCell<State>,
    pub state: &'a mut State,
    pub filtered_items: Vec<ParsedLine<'a>>,
    pub filter_views: Filters<Vec<String>>,
}

impl<'a> MainView<'a> {
    pub fn new(
        state: &'a mut State,
        filtered_items: Vec<ParsedLine<'a>>,
        filter_views: Filters<Vec<String>>,
    ) -> Self {
        Self {
            state,
            filtered_items,
            filter_views,
        }
    }

    pub fn draw<B>(&mut self, f: &mut Frame<B>)
    where
        B: Backend,
    {
        let selected_style = Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Percentage(80), Percentage(20)].as_ref())
            .split(f.size());

        let attr_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Percentage(50), Percentage(50)].as_ref())
            .split(chunks[1]);

        self.draw_attributes(f, selected_style, ActiveList::Contexts, attr_chunks[0]);
        self.draw_attributes(f, selected_style, ActiveList::Tags, attr_chunks[1]);

        let list_items: Vec<ListItem> = self
            .filtered_items
            .iter()
            .map(|state_item| {
                let sub_text = state_item.start_date.unwrap_or("");
                let lines = vec![
                    Spans::from(Span::styled(
                        &state_item.body[..],
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(if state_item.complete {
                                Modifier::CROSSED_OUT
                            } else {
                                Modifier::BOLD
                            }),
                    )),
                    Spans::from(Span::styled(sub_text, Style::default().fg(Color::DarkGray))),
                ];

                ListItem::new(lines)
            })
            .collect();

        let list = List::new(list_items)
            .block(
                Block::default()
                    .border_style(self.state.get_style(ActiveList::Tasks))
                    .borders(Borders::ALL)
                    .title("Tasks"),
            )
            .highlight_style(selected_style)
            .highlight_symbol("*");

        f.render_stateful_widget(list, chunks[0], &mut self.state.tasks.pos);
    }

    fn draw_attributes<B: Backend>(
        &mut self,
        f: &mut Frame<'_, B>,
        selected_style: Style,
        list_t: ActiveList,
        chunk: Rect,
    ) {
        let list_items: Vec<ListItem> = self
            .filter_views
            .get(list_t)
            .iter()
            .map(|i| ListItem::new(Span::raw(i)))
            .collect();

        let list = List::new(list_items)
            .block(
                Block::default()
                    .border_style(self.state.get_style(list_t))
                    .borders(Borders::ALL)
                    .title(list_t.to_string()),
            )
            .highlight_symbol("*")
            .highlight_style(selected_style);

        let block_state = self.state.get_state_mut(list_t);

        f.render_stateful_widget(list, chunk, &mut block_state.pos);
    }
}

impl<'a> EventHandler<Key> for MainView<'a> {
    fn handle(&mut self, event: Event<Key>) -> Option<Action> {
        match event {
            Event::Input(key) => match key {
                Key::Char('q') | Key::Ctrl('c') | Key::Ctrl('d') => return Some(Action::Exit),
                Key::Char('j') => self.state.next(),
                Key::Char('k') => self.state.previous(),
                Key::Char('l') => self.state.move_right(),
                Key::Char('h') => self.state.move_left(),
                Key::Char(' ') => {
                    let index = match self.state.active_list {
                        ActiveList::Tasks => self
                            .state
                            .tasks
                            .pos
                            .selected()
                            .map(|i| self.filtered_items[i].index),
                        ActiveList::Contexts => self.state.contexts.pos.selected(),
                        ActiveList::Tags => self.state.tags.pos.selected(),
                    };

                    if let Some(i) = index {
                        return Some(Action::Select(i));
                    }
                }
                Key::Char('w') => return Some(Action::Write),
                Key::Char('D') => {
                    if ActiveList::Tasks == self.state.active_list {
                        if let Some(i) = self.state.tasks.pos.selected() {
                            return Some(Action::Delete(self.filtered_items[i].index));
                        }
                    }
                }
                Key::Char('r') => return Some(Action::Reload),
                _ => {}
            },
            Event::_Tick => {}
        };
        None
    }
}
