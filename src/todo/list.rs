use super::item::Item;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct List {
    path: String,
    pub raw_items: Vec<String>,
}

impl List {
    pub fn new(path: String) -> Result<List, std::io::Error> {
        let file = File::open(&path)?;
        let mut buf_reader = BufReader::new(file);

        let mut items = Vec::new();

        loop {
            let mut line = String::new();
            match buf_reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => items.push(line),
                Err(e) => return Err(e),
            }
        }

        Ok(
            List{
                path,
                raw_items: items,
            }
        )
    }
}

fn print_line(line: String) {
    let i = Item::new(&line[..]);
    println!(">: {:?}", i);
}
