use crate::todo::{ListHandle, ListRep};
use crate::{
    app::{ActiveList, MainView, State},
    todo::ParsedLine,
};
use crate::{
    event::{Generator, Handler},
    filters::Filters,
};

use chrono::Utc;
use std::{collections::BTreeSet, error::Error, path::Path};
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::{Backend, TermionBackend},
    Terminal,
};

#[derive(Debug)]
pub enum Action {
    Select(usize),
    Write,
    Delete(usize),
    Reload,
    Exit,
}

pub fn start_term() -> Result<(), Box<dyn Error>> {
    let stdout = std::io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    run_with_term(&mut terminal)
}

fn run_with_term<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    const FILENAME: &str = "main.todo.txt";

    let eventgen = Generator::new();
    let todo_dir = Path::new(env!("TODO_DIR"));
    let todo_path = todo_dir.join(FILENAME);
    let list_handle = ListHandle::new(&todo_path);

    loop {
        match run_with_file(terminal, &list_handle, &eventgen)? {
            Action::Reload => {} // just continue
            Action::Exit => break Ok(()),
            action => panic!(format!("{:?} action unhandled at this stage", action)),
        };
    }
}

fn run_with_file<B: Backend>(
    terminal: &mut Terminal<B>,
    list_handle: &ListHandle,
    eventgen: &Generator,
) -> Result<Action, Box<dyn Error>> {
    let mut list_rep = ListRep::new(list_handle)?;

    let mut state = State::new(
        list_rep.tasks.len(),
        list_rep.filters.contexts.len(),
        list_rep.filters.tags.len(),
    );

    //let mut ctx_filters: BTreeSet<&str> = BTreeSet::new();
    //let mut tag_filters: BTreeSet<&str> = BTreeSet::new();
    let mut active_filters: Filters<BTreeSet<&str>> =
        Filters::new(BTreeSet::new(), BTreeSet::new());

    loop {
        let filtered_items: Vec<ParsedLine> = list_rep
            .tasks
            .iter()
            .enumerate()
            .filter_map(|(i, task)| {
                if active_filters.include(task) {
                    return Some(ParsedLine::new(&task[..], i));
                }
                None
            })
            .collect();

        state.tasks.reset(filtered_items.len());

        let main_view = MainView::new(
            &mut state,
            filtered_items,
            [
                make_view_strings(&list_rep.filters.contexts, &active_filters.contexts),
                make_view_strings(&list_rep.filters.tags, &active_filters.tags),
            ],
        );

        let action = run_with_view(terminal, eventgen, main_view)?;

        match action {
            Action::Select(i) => match state.active_list {
                ActiveList::Tasks => {
                    list_rep.tasks[i] = if list_rep.tasks[i].starts_with("x ") {
                        list_rep.tasks[i]
                            .splitn(3, ' ')
                            .nth(2)
                            .expect("what")
                            .to_owned()
                    } else {
                        let dt = Utc::today();
                        format!("x {} {}", dt.format("%Y-%m-%d"), list_rep.tasks[i])
                    };
                    list_rep.modified = true;
                }

                active_list @ ActiveList::Contexts | active_list @ ActiveList::Tags => {
                    let filters = active_filters.get_mut(active_list);
                    let filter_source = list_rep.filters.get(active_list);
                    if filters.contains(&filter_source[i][..]) {
                        filters.remove(&filter_source[i][..]);
                    } else {
                        filters.insert(&filter_source[i]);
                    }
                }
            },
            Action::Write => {
                if list_rep.modified {
                    list_handle.write(&list_rep.tasks)?;
                    list_rep.modified = false;
                }
            }
            Action::Delete(i) => {
                if ActiveList::Tasks == state.active_list {
                    list_rep.tasks.remove(i);
                    list_rep.modified = true;
                }
            }
            action => return Ok(action),
        }
    }
}

fn run_with_view<B: Backend>(
    terminal: &mut Terminal<B>,
    eventgen: &Generator,
    mut main_view: MainView,
) -> Result<Action, Box<dyn Error>> {
    let res = loop {
        terminal.draw(|f| main_view.draw(f))?;

        if let Some(action) = main_view.handle(eventgen.next()?) {
            break action;
        }
    };

    Ok(res)
}

fn make_view_strings(input_list: &[String], filters: &BTreeSet<&str>) -> Vec<String> {
    input_list
        .iter()
        .map(|v| {
            format!(
                "[{}] {}",
                if filters.contains(&v[..]) { "x" } else { " " },
                &v[1..],
            )
        })
        .collect()
}
