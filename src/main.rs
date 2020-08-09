mod event;
mod table;
mod todo;

use event::{Event, Events};
use std::error::Error;
use std::iter;

use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use table::TodoListTable;
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout, Direction},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table},
    Terminal,
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
    let mut tlt = TodoListTable::new()?;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(70),
                        Constraint::Percentage(30),
                    ]
                    .as_ref(),
                )
                .split(f.size());



            let selected_style = Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD);
            let normal_style = Style::default().fg(Color::White);
            let header = ["Header1"];
            let rows = tlt
                .list.raw_items
                .iter()
                .map(|i| Row::StyledData(iter::once(i), normal_style));

            let t = Table::new(header.iter(), rows)
                .block(Block::default().borders(Borders::ALL).title("Tasks"))
                .highlight_style(selected_style)
                .highlight_symbol("*")
                .widths(&[
                    Constraint::Percentage(100),
                ]);
            f.render_stateful_widget(t, chunks[0], &mut tlt.state);


            let block = Block::default().title("Block 2").borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
        })?;

        if let Event::Input(key) = events.next()? {
            match key {
                Key::Char('q') => break,
                Key::Char('j') => tlt.next(),
                Key::Char('k') => tlt.previous(),
                Key::Up => tlt.list.raw_items.push(String::from("new item")),
                _ => {}
            }
        }
    }

    Ok(())
}

