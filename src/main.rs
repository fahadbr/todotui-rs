mod event;
mod table;
mod todo;

use event::{Event, Events};
use std::error::Error;
use todo::ParsedItem;

use table::StatefulTodoList;
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
    let mut stlist = StatefulTodoList::new()?;
    let selected_style = Style::default()
        .fg(Color::Green)
        .add_modifier(Modifier::BOLD);
    let normal_style = Style::default().fg(Color::White);

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

            draw_attributes(f, &stlist, normal_style, selected_style, attr_chunks);

            let list_items: Vec<ListItem> = stlist
                .list
                .raw_items
                .iter()
                .map(|i| {
                    let parsed_item = ParsedItem::new(&i[..]);
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

                    ListItem::new(lines)
                })
                .collect();

            let list = List::new(list_items)
                .block(Block::default().borders(Borders::ALL).title("Tasks"))
                .highlight_style(selected_style)
                .highlight_symbol("*");

            f.render_stateful_widget(list, chunks[0], &mut stlist.state);
        })?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') | Key::Ctrl('c') | Key::Ctrl('d') => break,
                Key::Char('j') => stlist.next(),
                Key::Char('k') => stlist.previous(),
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
    tlt: &StatefulTodoList,
    normal_style: Style,
    selected_style: Style,
    chunks: Vec<Rect>,
) {
    {
        let list_items: Vec<ListItem> = tlt
            .list
            .contexts
            .iter()
            .map(|i| ListItem::new(Span::raw(i)))
            .collect();

        let list = List::new(list_items)
            .block(Block::default().borders(Borders::ALL).title("Contexts"))
            .highlight_style(selected_style);
        f.render_widget(list, chunks[0]);
    }

    {
        let list_items: Vec<ListItem> = tlt
            .list
            .tags
            .iter()
            .map(|i| ListItem::new(Span::raw(i)))
            .collect();

        let list = List::new(list_items)
            .block(Block::default().borders(Borders::ALL).title("Tags"))
            .highlight_style(selected_style);
        f.render_widget(list, chunks[1]);
    }
}
