use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ParsedLine<'a> {
    pub raw: &'a str,
    pub index: usize,
    pub body: String,
    pub complete: bool,
    pub start_date: Option<&'a str>,
    pub completion_date: Option<&'a str>,
    pub due_date: Option<&'a str>,
    pub threshold_date: Option<&'a str>,
    pub hidden: bool,
    pub priority: Option<char>,
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

impl<'a> ParsedLine<'a> {
    pub fn new(raw: &'a str, index: usize) -> Self {
        let x: &[char] = &['\n', '\r'];
        let mut raw = raw.trim_end_matches(x);

        let mut item = Self {
            raw,
            index,
            body: String::with_capacity(raw.len()),
            complete: false,
            start_date: None,
            completion_date: None,
            due_date: None,
            threshold_date: None,
            hidden: false,
            priority: None,
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
        let mut body_appended = false;
        for word in raw.split_whitespace() {
            if !body_appended && parse_state == Parse::Body {
                item.body.push_str(&raw[body_start_idx..]);
                body_appended = true;
            } else {
                body_start_idx += word.len() + 1;
            }

            item.process_word(&mut parse_state, word);
        }

        item
    }

    fn is_priority(word: &'a str) -> Option<char> {
        if word.len() != 3 {
            return None;
        }

        let mut priority: Option<char> = None;
        for (i, c) in word.chars().enumerate() {
            match i {
                0 if c != '(' => return None,
                1 => priority = Some(c),
                2 if c != ')' => return None,
                _ => continue,
            }
        }

        priority
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
        match word.chars().next() {
            Some('@') => self.contexts.push(word),
            Some('+') => self.tags.push(word),
            Some(_) => {
                let things: Vec<&str> = word.splitn(2, ':').collect();
                if things.len() != 2 {
                    return;
                }
                let key = things[0];
                let val = things[1];
                match key {
                    "due" if ParsedLine::is_date(val) => self.due_date = Some(val),
                    "t" if ParsedLine::is_date(val) => self.threshold_date = Some(val),
                    "h" if val == "1" => self.hidden = true,
                    "rec" => self.recurrance = Some(val),
                    _ => self.extensions.push((key, val)),
                }
            }
            None => {}
        }
    }

    fn process_word(&mut self, parse_state: &mut Parse, word: &'a str) {
        loop {
            match parse_state {
                Parse::CompletionDate => {
                    *parse_state = Parse::Priority;
                    if ParsedLine::is_date(word) {
                        self.completion_date = Some(word);
                        return;
                    }
                }
                Parse::Priority => {
                    *parse_state = Parse::StartDate;
                    if let Some(p) = ParsedLine::is_priority(word) {
                        self.priority = Some(p);

                        self.body.push('(');
                        self.body.push(p);
                        self.body.push(')');
                        self.body.push(' ');
                        return;
                    }
                }
                Parse::StartDate => {
                    *parse_state = Parse::Body;
                    if ParsedLine::is_date(word) {
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
}
