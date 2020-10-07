use super::item::ParsedLine;

use std::io::{prelude::*, Error};
use std::{collections::BTreeSet, fs, io::BufReader};
use std::{fs::File, path::Path};

pub struct Rep {
    pub tasks: Vec<String>,
    pub contexts: Vec<String>,
    pub tags: Vec<String>,
    pub modified: bool,
}

impl Rep {
    pub fn new(handle: &Handle) -> Result<Rep, Error> {
        let items = handle.get_lines()?;
        let mut contexts = BTreeSet::new();
        let mut tags = BTreeSet::new();

        for line in &items {
            let i = ParsedLine::new(&line);
            for c in i.contexts {
                contexts.insert(c);
            }
            for t in i.tags {
                tags.insert(t);
            }
        }

        let list = Rep {
            contexts: contexts.into_iter().map(str::to_string).collect(),
            tags: tags.into_iter().map(str::to_string).collect(),
            tasks: items,
            modified: false,
        };

        Ok(list)
    }
}

pub struct Handle<'a> {
    path: &'a Path,
}

impl<'a> Handle<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self { path }
    }

    pub fn get_lines(&self) -> Result<Vec<String>, Error> {
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

    pub fn write(&self, lines: &[String]) -> Result<(), Error> {
        let mut tmp_file_path = self.path.parent().expect("betta be hea").to_owned();
        tmp_file_path.push(".todotuirs.tmp");
        let mut tmp_file = File::create(&tmp_file_path)?;

        for line in lines {
            match writeln!(tmp_file, "{}", line) {
                Ok(_) => {}

                // TODO: remove temporary file on error
                Err(e) => return Err(e),
            };
        }

        fs::rename(&tmp_file_path, self.path)?;

        Ok(())
    }
}
