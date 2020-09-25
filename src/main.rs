mod app;
mod event;
mod flags;
mod state;
mod todo;

use event::{Event, Events};
use std::{collections::HashSet, error::Error};
use todo::ParsedItem;

use state::{ActiveList, State};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::{Backend, TermionBackend},
    layout::{Constraint::Percentage, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame, Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    flags::parse();
    //read_file()
    start_term()
}

fn start_term() -> Result<(), Box<dyn Error>> {
    let stdout = std::io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();
    let mut filters = HashSet::new();
    let mut app = State::new()?;
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

            draw_attributes(f, &mut app, selected_style, attr_chunks);

            let mut list_items =  Vec::new();
            for state_item in &app.list.raw_items {

                if !filters.is_empty() {
                    if let None = filters.iter().find(|x: &&String| {state_item.contains(&x[..])}) {
                        continue;
                    }
                }

                let parsed_item = ParsedItem::new(&state_item[..]);
                let sub_text = parsed_item.start_date.unwrap_or("");
                let lines = vec![
                    Spans::from(Span::styled(
                        parsed_item.body,
                        Style::default().fg(Color::White).add_modifier(
                            if parsed_item.complete {
                                Modifier::CROSSED_OUT
                            } else {
                                Modifier::BOLD
                            },
                        ),
                    )),
                    Spans::from(Span::styled(sub_text, Style::default().fg(Color::DarkGray))),
                ];

                list_items.push(ListItem::new(lines));
            }


            let list = List::new(list_items)
                .block(
                    Block::default()
                        .border_style(app.get_style(ActiveList::Tasks))
                        .borders(Borders::ALL)
                        .title("Tasks"),
                )
                .highlight_style(selected_style)
                .highlight_symbol("*");

            f.render_stateful_widget(list, chunks[0], &mut app.task_state);
        })?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') | Key::Ctrl('c') | Key::Ctrl('d') => break,
                Key::Char('j') => app.next(),
                Key::Char('k') => app.previous(),
                Key::Char('l') => app.move_right(),
                Key::Char('h') => app.move_left(),
                Key::Char(' ') => app.select(&mut filters),
                //Key::Up => tlt.list.raw_items.push(String::from("new item")),
                _ => {}
            },
            Event::Tick => continue,
        }
    }

    Ok(())
}

fn draw_attributes<B: Backend>(
    f: &mut Frame<'_, B>,
    state: &mut State,
    selected_style: Style,
    chunks: Vec<Rect>,
) {
    {
        let list_items: Vec<ListItem> = state
            .list
            .contexts
            .iter()
            .map(|i| ListItem::new(Span::raw(i)))
            .collect();

        let list = List::new(list_items)
            .block(
                Block::default()
                    .border_style(state.get_style(ActiveList::Contexts))
                    .borders(Borders::ALL)
                    .title("Contexts"),
            )
            .highlight_symbol("*")
            .highlight_style(selected_style);
        f.render_stateful_widget(list, chunks[0], &mut state.context_state);
    }

    {
        let list_items: Vec<ListItem> = state
            .list
            .tags
            .iter()
            .map(|i| ListItem::new(Span::raw(i)))
            .collect();

        let list = List::new(list_items)
            .block(
                Block::default()
                    .border_style(state.get_style(ActiveList::Tags))
                    .borders(Borders::ALL)
                    .title("Tags"),
            )
            .highlight_symbol("*")
            .highlight_style(selected_style);
        f.render_stateful_widget(list, chunks[1], &mut state.tag_state);
    }
}
