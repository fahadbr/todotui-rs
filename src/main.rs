mod event;
mod table;
mod todo;

use event::{Event, Events};
use std::error::Error;
use std::iter;
use todo::Item;

use table::TodoListTable;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
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
    let selected_style = Style::default()
        .fg(Color::Green)
        .add_modifier(Modifier::BOLD);
    let normal_style = Style::default().fg(Color::White);

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(f.size());

            {
                let empty_item = Item::new("");
                let item = tlt.get_item().unwrap_or(empty_item);
                let row_data = item.to_row_data();
                let rows = row_data
                    .iter()
                    .map(|i| Row::StyledData(i.iter(), normal_style));
                let attr_table = Table::new(["Contexts", "Tags", "Priority"].iter(), rows)
                    .block(Block::default().borders(Borders::ALL).title("Attributes"))
                    .highlight_style(selected_style)
                    .widths(&[Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(34)]);
                f.render_widget(attr_table, chunks[0]);
            }
            let header = ["Header1"];
            let rows = tlt
                .list
                .raw_items
                .iter()
                .map(|i| Row::StyledData(iter::once(i), normal_style));

            let main_table = Table::new(header.iter(), rows)
                .block(Block::default().borders(Borders::ALL).title("Tasks"))
                .highlight_style(selected_style)
                .highlight_symbol("*")
                .widths(&[Constraint::Percentage(100)]);
            f.render_stateful_widget(main_table, chunks[1], &mut tlt.state);
        })?;

        if let Event::Input(key) = events.next()? {
            match key {
                Key::Char('q') | Key::Ctrl('c') | Key::Ctrl('d') => break,
                Key::Char('j') => tlt.next(),
                Key::Char('k') => tlt.previous(),
                //Key::Up => tlt.list.raw_items.push(String::from("new item")),
                _ => {}
            }
        }
    }

    Ok(())
}
