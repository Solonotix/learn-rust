use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct Message {
    pub from: String,
    pub contents: String
}

impl Message {
    pub fn new(from: String, contents: &str) -> Self {
        Message { from: String::from(from), contents: String::from(contents) }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.contents)
    }
}