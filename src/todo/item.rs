use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ParsedItem<'a> {
    pub raw: &'a str,
    pub body: &'a str,
    pub complete: bool,
    pub start_date: Option<&'a str>,
    pub completion_date: Option<&'a str>,
    pub due_date: Option<&'a str>,
    pub threshold_date: Option<&'a str>,
    pub hidden: bool,
    pub priority: char,
    pub contexts: Vec<&'a str>,
    pub tags: Vec<&'a str>,
    pub recurrance: Option<&'a str>,
    pub extensions: Vec<(&'a str, &'a str)>,
}

#[derive(Debug, PartialEq, Eq)]
enum Parse {
    CompletionDate,
    Priority,
    StartDate,
    Body,
}

impl Display for Parse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let rep = match self {
            Parse::CompletionDate => "CompletionDate",
            Parse::Priority => "Priority",
            Parse::StartDate => "StartDate",
            Parse::Body => "Body",
        };
        write!(f, "{}", rep)
    }
}

impl<'a> ParsedItem<'a> {
    pub fn new(raw: &'a str) -> Self {
        let x: &[char] = &['\n', '\r'];
        let mut raw = raw.trim_end_matches(x);

        let mut item = Self {
            raw,
            body: "",
            complete: false,
            start_date: None,
            completion_date: None,
            due_date: None,
            threshold_date: None,
            hidden: false,
            priority: '\0',
            contexts: Vec::new(),
            tags: Vec::new(),
            recurrance: None,
            extensions: Vec::new(),
        };


        if raw.starts_with('x') {
            raw = raw.trim_start_matches('x').trim_start();
            item.complete = true;
        }


        let mut parse_state = if item.complete {
            Parse::CompletionDate
        } else {
            Parse::Priority
        };


        let mut body_start_idx = 0;
        for word in raw.split_whitespace() {
            if item.body == "" && parse_state == Parse::Body {
                item.body = &raw[body_start_idx..];
            } else {
                body_start_idx += word.len() + 1;
            }

            item.process_word(&mut parse_state, word);
        }

        item
    }

    fn is_priority(word: &'a str) -> Result<char, ()> {
        if word.len() != 3 {
            return Err(());
        }

        let mut priority: char = '\0';
        for (i, c) in word.chars().enumerate() {
            match i {
                0 if c != '(' => return Err(()),
                1 => priority = c,
                2 if c != ')' => return Err(()),
                _ => continue,
            }
        }

        return Ok(priority);
    }

    fn is_date(word: &'a str) -> bool {
        if word.len() != 10 {
            return false;
        }

        for (index, c) in word.chars().enumerate() {
            match index {
                4 | 7 => {
                    if c != '-' {
                        return false;
                    }
                }
                _ => {
                    if c < '0' || c > '9' {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn process_body(&mut self, word: &'a str) {
        let chars: Vec<char> = word.chars().collect();
        if chars.len() == 0 {
            return;
        }

        // TODO: finish implementing
        match chars[0] {
            '@' => self
                .contexts
                .push(word.strip_prefix("@").expect("couldn't trip @ symbol")),
            '+' => self
                .tags
                .push(word.strip_prefix("+").expect("couldn't trip + symbol")),
            _ => {
                let things: Vec<&str> = word.splitn(2, ":").collect();
                if things.len() != 2 {
                    return;
                }
                let key = things[0];
                let val = things[1];
                match key {
                    "due" if ParsedItem::is_date(val) => self.due_date = Some(val),
                    "t" if ParsedItem::is_date(val) => self.threshold_date = Some(val),
                    "h" if val == "1" => self.hidden = true,
                    "rec" => self.recurrance = Some(val),
                    _ => self.extensions.push((key, val)),
                }
            }
        }
    }

    fn process_word(&mut self, parse_state: &mut Parse, word: &'a str) {
        loop {
            match parse_state {
                Parse::CompletionDate => {
                    *parse_state = Parse::Priority;
                    if ParsedItem::is_date(word) {
                        self.completion_date = Some(word);
                        return;
                    }
                }
                Parse::Priority => {
                    *parse_state = Parse::StartDate;
                    match ParsedItem::is_priority(word) {
                        Ok(priority) => {
                            self.priority = priority;
                            return;
                        }
                        Err(_) => {}
                    }
                }
                Parse::StartDate => {
                    *parse_state = Parse::Body;
                    if ParsedItem::is_date(word) {
                        self.start_date = Some(word);
                        return;
                    }
                }
                Parse::Body => {
                    self.process_body(word);
                    return;
                }
            }
        }
    }

    pub fn to_row_data(&self) -> Vec<Vec<String>> {
        let mut max_length = 0;
        if self.contexts.len() > max_length {
            max_length = self.contexts.len();
        }
        if self.tags.len() > max_length {
            max_length = self.tags.len();
        }

        let mut rows = Vec::with_capacity(max_length);
        for i in 0..max_length {
            let mut cols = Vec::with_capacity(3);
            cols.push(String::from(*self.contexts.get(i).unwrap_or(&"")));
            cols.push(String::from(*self.tags.get(i).unwrap_or(&"")));
            cols.push(self.priority.to_string());
            rows.push(cols);
        }

        rows
    }
}
