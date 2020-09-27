use super::item::ParsedItem;

use std::io::prelude::*;
use std::{collections::BTreeSet, io::BufReader, path::PathBuf};
use std::{fs::File, path::Path};

pub struct ListRep<'a> {
    pub raw_items: Vec<&'a str>,
    pub contexts: Vec<FilterOpt<'a>>,
    pub tags: Vec<FilterOpt<'a>>,
}

//impl Default for List {
//fn default() -> Self {
//const FILENAME: &'static str = "main.todo.txt";
//let todo_dir = Path::new(env!("TODO_DIR"));
//let todo_path = todo_dir.join(FILENAME);
//Self::new(todo_path).expect("failed to find todotxt file")
//}
//}

impl<'a> ListRep<'a> {
    pub fn new( handle: &'a ListHandle) -> ListRep<'a> {

        let mut items = Vec::new();
        let mut contexts = BTreeSet::new();
        let mut tags = BTreeSet::new();


        for line in handle.lines.iter() {
            let i = ParsedItem::new(&line);
            items.push(&line[..]);
            for c in i.contexts.into_iter() {
                contexts.insert(c);
            }
            for t in i.tags.into_iter() {
                tags.insert(t);
            }
        }

        let list = ListRep {
            contexts: contexts.into_iter().map(|i| FilterOpt::new(i)).collect(),
            tags: tags.into_iter().map(|i| FilterOpt::new(i)).collect(),
            raw_items: items,
        };

        list
    }
}

pub struct ListHandle {
    path: PathBuf,
    lines: Vec<String>,
}

impl ListHandle {
    pub fn new(path: PathBuf) -> Result<Self, std::io::Error> {
        let file = File::open(&path)?;
        let buf_reader = BufReader::new(file);

        let mut lines = Vec::new();

        for line_res in buf_reader.lines() {
            match line_res {
                Ok(line) => {
                    lines.push(line);
                }
                Err(e) => return Err(e),
            }
        }

        Ok(Self { path, lines })
    }
}

#[derive(Debug)]
pub struct FilterOpt<'a> {
    pub val: &'a str,
    pub selected: bool,
    pub selected_str: String,
}

impl<'a> FilterOpt<'a> {
    pub fn new(val: &'a str) -> Self {
        Self {
            selected: false,
            selected_str: Self::make_selected_str(false, val),
            val,
        }
    }

    pub fn toggle_select(&mut self) {
        self.selected = !self.selected;
        self.selected_str = Self::make_selected_str(self.selected, &self.val);
    }

    fn make_selected_str(selected: bool, val: &'a str) -> String {
        format!("[{}] {}", if selected { "x" } else { " " }, &val[1..])
    }
}
