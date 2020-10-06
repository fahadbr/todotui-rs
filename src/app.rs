use crate::event::{Event, Events};
use crate::state::{ActiveList, State};
use crate::todo::{ListHandle, ListRep, ParsedLine};

use std::{collections::BTreeSet, error::Error, path::Path};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::{Backend, TermionBackend},
    layout::{Constraint::Percentage, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame, Terminal,
};
use chrono::Utc;

enum Action {
    Select(usize),
    Reload,
    Exit,
}

pub struct FilterViews {
    pub contexts: Vec<String>,
    pub tags: Vec<String>,
}

pub struct TaskRep<'a> {
    pub index: usize,
    pub val: &'a str,
}

pub fn start_term() -> Result<(), Box<dyn Error>> {
    let stdout = std::io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    run_with_term(&mut terminal)
}

/*
 * term
 *
 * filehandle
 *
 * vec of tasks
 *
 * vec of ctx
 * vec of tag
 *
 * ctx state
 * tag state
 * applied filters
 *
 * filtered items
 * task state
 *
 */

fn run_with_term<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    const FILENAME: &str = "main.todo.txt";

    let events = Events::new();
    let todo_dir = Path::new(env!("TODO_DIR"));
    let todo_path = todo_dir.join(FILENAME);
    let list_handle = ListHandle::new(&todo_path);

    run_with_file(terminal, &list_handle, &events)
}

fn run_with_file<B: Backend>(
    terminal: &mut Terminal<B>,
    list_handle: &ListHandle,
    events: &Events,
) -> Result<(), Box<dyn Error>> {
    let mut list_rep = ListRep::new(list_handle)?;

    let mut state = State::new(
        list_rep.tasks.len(),
        list_rep.contexts.len(),
        list_rep.tags.len(),
    );

    let mut ctx_filters: BTreeSet<&str> = BTreeSet::new();
    let mut tag_filters: BTreeSet<&str> = BTreeSet::new();

    loop {
        let filtered_items: Vec<TaskRep> = list_rep
            .tasks
            .iter()
            .enumerate()
            .filter_map(|(i, task)| {
                if !ctx_filters.is_empty() && None == ctx_filters.iter().find(|&f| task.contains(f))
                {
                    return None;
                }
                if !tag_filters.is_empty() && None == tag_filters.iter().find(|&f| task.contains(f))
                {
                    return None;
                }
                Some(TaskRep {
                    index: i,
                    val: &task[..],
                })
            })
            .collect();

        let filter_view = FilterViews {
            contexts: make_view(&list_rep.contexts, &ctx_filters),
            tags: make_view(&list_rep.tags, &tag_filters),
        };

        state.task_state.reset(filtered_items.len());

        let action = run_with_view(terminal, events, &mut state, &filter_view, &filtered_items)?;

        match action {
            Action::Select(i) => match state.active_list {
                ActiveList::Tasks => {
                    if list_rep.tasks[i].starts_with("x ") {
                        list_rep.tasks[i] = list_rep.tasks[i]
                            .splitn(3, ' ')
                            .nth(2)
                            .expect("what")
                            .to_owned();
                            //.strip_prefix("x ")
                            //.expect("what")
                            //.to_owned();
                    } else {
                        let dt = Utc::today();
                        list_rep.tasks[i] = format!("x {} {}", dt.format("%Y-%m-%d"), list_rep.tasks[i]);
                    }
                }

                ActiveList::Contexts => {
                    if ctx_filters.contains(&list_rep.contexts[i][..]) {
                        ctx_filters.remove(&list_rep.contexts[i][..]);
                    } else {
                        ctx_filters.insert(&list_rep.contexts[i]);
                    }
                }
                ActiveList::Tags => {
                    if tag_filters.contains(&list_rep.tags[i][..]) {
                        tag_filters.remove(&list_rep.tags[i][..]);
                    } else {
                        tag_filters.insert(&list_rep.tags[i]);
                    }
                }
            },
            Action::Reload => {}
            Action::Exit => return Ok(()),
        }
    }
}

fn run_with_view<B: Backend>(
    terminal: &mut Terminal<B>,
    events: &Events,
    state: &mut State,
    filter_views: &FilterViews,
    filtered_items: &[TaskRep],
) -> Result<Action, Box<dyn Error>> {
    let selected_style = Style::default()
        .fg(Color::Green)
        .add_modifier(Modifier::BOLD);

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Percentage(80), Percentage(20)].as_ref())
                .split(f.size());

            let attr_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Percentage(50), Percentage(50)].as_ref())
                .split(chunks[1]);

            draw_attributes(
                f,
                state,
                &filter_views.contexts,
                selected_style,
                ActiveList::Contexts,
                attr_chunks[0],
            );

            draw_attributes(
                f,
                state,
                &filter_views.tags,
                selected_style,
                ActiveList::Tags,
                attr_chunks[1],
            );

            let mut list_items = Vec::new();
            for state_item in filtered_items {
                let parsed_item = ParsedLine::new(&state_item.val[..]);
                let sub_text = parsed_item.start_date.unwrap_or("");
                let lines = vec![
                    Spans::from(Span::styled(
                        parsed_item.body,
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(if parsed_item.complete {
                                Modifier::CROSSED_OUT
                            } else {
                                Modifier::BOLD
                            }),
                    )),
                    Spans::from(Span::styled(sub_text, Style::default().fg(Color::DarkGray))),
                ];

                list_items.push(ListItem::new(lines));
            }

            let list = List::new(list_items)
                .block(
                    Block::default()
                        .border_style(state.get_style(ActiveList::Tasks))
                        .borders(Borders::ALL)
                        .title("Tasks"),
                )
                .highlight_style(selected_style)
                .highlight_symbol("*");

            f.render_stateful_widget(list, chunks[0], &mut state.task_state.state);
        })?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') | Key::Ctrl('c') | Key::Ctrl('d') => break,
                Key::Char('j') => state.next(),
                Key::Char('k') => state.previous(),
                Key::Char('l') => state.move_right(),
                Key::Char('h') => state.move_left(),
                Key::Char(' ') => {
                    let index = match state.active_list {
                        ActiveList::Tasks => state
                            .task_state
                            .state
                            .selected()
                            .map(|i| filtered_items[i].index),
                        ActiveList::Contexts => state.context_state.state.selected(),
                        ActiveList::Tags => state.tag_state.state.selected(),
                    };

                    if let Some(i) = index {
                        return Ok(Action::Select(i));
                    }
                }
                //Key::Up => tlt.list.raw_items.push(String::from("new item")),
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(Action::Exit)
}

fn draw_attributes<B: Backend>(
    f: &mut Frame<'_, B>,
    state: &mut State,
    filter_opts: &[String],
    selected_style: Style,
    list_t: ActiveList,
    chunk: Rect,
) {
    let list_items: Vec<ListItem> = filter_opts
        .iter()
        .map(|i| ListItem::new(Span::raw(i)))
        .collect();

    let list = List::new(list_items)
        .block(
            Block::default()
                .border_style(state.get_style(list_t))
                .borders(Borders::ALL)
                .title(list_t.to_string()),
        )
        .highlight_symbol("*")
        .highlight_style(selected_style);

    f.render_stateful_widget(list, chunk, &mut state.get_state_mut(list_t).state);
}

fn make_view(input_list: &[String], filters: &BTreeSet<&str>) -> Vec<String> {
    input_list
        .iter()
        .map(|v| {
            format!(
                "[{}] {}",
                if filters.contains(&v[..]) { "x" } else { " " },
                v,
            )
        })
        .collect()
}
