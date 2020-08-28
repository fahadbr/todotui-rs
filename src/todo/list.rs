use super::item::ParsedItem;

use std::fs::File;
use std::io::prelude::*;
use std::{collections::BTreeSet, io::BufReader, path::PathBuf};

pub struct List {
    path: PathBuf,
    pub raw_items: Vec<String>,
    pub contexts: BTreeSet<String>,
    pub tags: BTreeSet<String>,
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
            contexts: contexts.into_iter().map(|i| String::from(i)).collect(),
            tags: tags.into_iter().map(|i| String::from(i)).collect(),
            raw_items: items,
        };

        Ok(list)
    }
}
