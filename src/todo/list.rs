use super::item::ParsedItem;

use std::io::prelude::*;
use std::{collections::BTreeSet, io::BufReader};
use std::{fs::File, path::Path};

pub struct ListRep {
    pub tasks: Vec<String>,
    pub contexts: Vec<String>,
    pub tags: Vec<String>,
}

impl ListRep {
    pub fn new(handle: &ListHandle) -> Result<ListRep, std::io::Error> {
        let items = handle.get_lines()?;
        let mut contexts = BTreeSet::new();
        let mut tags = BTreeSet::new();

        for line in items.iter() {
            let i = ParsedItem::new(&line);
            for c in i.contexts.into_iter() {
                contexts.insert(c);
            }
            for t in i.tags.into_iter() {
                tags.insert(t);
            }
        }

        let list = ListRep {
            contexts: contexts.into_iter().map(|s| s.to_string()).collect(),
            tags: tags.into_iter().map(|s| s.to_string()).collect(),
            tasks: items,
        };

        Ok(list)
    }
}

pub struct ListHandle<'a> {
    path: &'a Path,
}

impl<'a> ListHandle<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self { path }
    }

    pub fn get_lines(&self) -> Result<Vec<String>, std::io::Error> {
        let file = File::open(self.path)?;
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

        Ok(lines)
    }
}

