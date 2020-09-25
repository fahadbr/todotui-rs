use super::item::ParsedItem;

use std::fs::File;
use std::io::prelude::*;
use std::{collections::BTreeSet, io::BufReader, path::PathBuf};

pub struct List {
    path: PathBuf,
    pub raw_items: Vec<String>,
    pub contexts: Vec<FilterOpt>,
    pub tags: Vec<FilterOpt>,
}

impl List {
    pub fn new(path: PathBuf) -> Result<List, std::io::Error> {
        let file = File::open(&path)?;
        let buf_reader = BufReader::new(file);

        let mut items = Vec::new();
        let mut contexts = BTreeSet::new();
        let mut tags = BTreeSet::new();

        for line_res in buf_reader.lines() {
            match line_res {
                Ok(line) => {
                    items.push(line);
                }
                Err(e) => return Err(e),
            }
        }

        for line in items.iter() {
            let i = ParsedItem::new(&line[..]);
            for c in i.contexts.into_iter() {
                contexts.insert(c);
            }
            for t in i.tags.into_iter() {
                tags.insert(t);
            }
        }

        let list = List {
            path,
            contexts: contexts
                .into_iter()
                .map(|i| FilterOpt::new(i.to_owned(), "@"))
                .collect(),
            tags: tags
                .into_iter()
                .map(|i| FilterOpt::new(i.to_owned(), "+"))
                .collect(),
            raw_items: items,
        };

        Ok(list)
    }
}

#[derive(Debug)]
pub struct FilterOpt {
    pub val: String,
    pub selected: bool,
    pub selected_str: String,
    pub prefixed_val: String,
    prefix: &'static str,
}

impl FilterOpt {
    pub fn new(val: String, prefix: &'static str) -> Self {
        Self {
            selected: false,
            selected_str: Self::make_selected_str(false, &val),
            prefixed_val: format!("{}{}", prefix, val),
            prefix,
            val,
        }
    }

    pub fn toggle_select(&mut self) -> String {
        self.selected = !self.selected;
        self.selected_str = Self::make_selected_str(self.selected, &self.val);
        self.prefixed_val.clone()
    }

    fn make_selected_str(selected: bool, val: &String) -> String {
        format!("[{}] {}", if selected { "x" } else { " " }, val)
    }
}
