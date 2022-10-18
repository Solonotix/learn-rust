use crate::forward::Forward;
use crate::message::Message;

#[derive(Clone)]
pub enum Chat {
    Message(Message),
    Forward(Forward)
}