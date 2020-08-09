use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Item<'a> {
    pub raw: &'a str,
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

#[derive(Debug)]
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

impl<'a> Item<'a> {
    pub fn new(raw: &'a str) -> Self {
        let x: &[char] = &['\n', '\r'];
        let mut raw = raw.trim_end_matches(x);

        let mut item = Self {
            raw,
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

        for word in raw.split_whitespace() {
            item.process_word(&mut parse_state, word);
        }

        item
    }

    fn is_pri(word: &'a str) -> Result<char, ()> {
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
            '@' => self.contexts.push(word.trim_start_matches("@")),
            '+' => self.tags.push(word.trim_start_matches("+")),
            _ => {
                let things: Vec<&str> = word.splitn(2, ":").collect();
                if things.len() != 2 {
                    return;
                }
                let key = things[0];
                let val = things[1];
                match key {
                    "due" if Item::is_date(val) => self.due_date = Some(val),
                    "t" if Item::is_date(val) => self.threshold_date = Some(val),
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
                    if Item::is_date(word) {
                        self.completion_date = Some(word);
                        return;
                    }
                }
                Parse::Priority => {
                    *parse_state = Parse::StartDate;
                    match Item::is_pri(word) {
                        Ok(priority) => {
                            self.priority = priority;
                            return;
                        }
                        Err(_) => {}
                    }
                }
                Parse::StartDate => {
                    *parse_state =  Parse::Body;
                    if Item::is_date(word) {
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
