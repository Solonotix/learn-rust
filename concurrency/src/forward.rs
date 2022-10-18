use crate::message::Message;

#[derive(Clone)]
pub struct Forward {
    pub from: String,
    pub contents: Message
}

impl Forward {
    pub fn new(from: String, contents: Message) -> Self {
        Forward { from, contents }
    }
}
