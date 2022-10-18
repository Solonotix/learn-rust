use std::collections::HashMap;
use std::fmt::{Debug,Display};
use std::sync::mpsc::{Receiver,SyncSender};

use rand::prelude::*;

use crate::chat::Chat;
use crate::forward::Forward;
use crate::message::Message;

pub struct User {
    limit: u8,
    pub name: String,
    receiver: Receiver<Chat>,
    sent: u8,
    transmitters: HashMap<String, SyncSender<Chat>>
}

unsafe impl Sync for User {

}

impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User name:{}", self.name)
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User name:{}", self.name)
    }
}

impl User {
    pub fn new(name: String, limit: u8, receiver: Receiver<Chat>) -> Self {
        User { name, limit, receiver, sent: 0, transmitters: HashMap::new() }
    }

    pub fn add(&mut self, to: String, transmitter: SyncSender<Chat>) {
        self.transmitters.insert(to, transmitter);
    }

    fn forward(&self, msg: Message) {
        let from = String::from(&self.name);
        let mut rng = thread_rng();
        match self.transmitters.iter().choose(&mut rng) {
            Some((to, tx)) => match tx.send(Chat::Forward(Forward::new(from, msg))) {
                Ok(_) => println!("{} sent message to {}", self.name, to),
                Err(_) => println!("Failed to send")
            },
            _ => println!("No transmitters available")
        };
    }

    pub fn is_active(&self) -> bool {
        self.limit > self.sent
    }

    pub fn logout(&self) {
        for name in self.transmitters.keys() {
            if let Some(tx) = self.transmitters.get(name) {
                let msg = Message {
                    from: self.name.to_string(),
                    contents: format!("Cya {}. I'm logging out after sending {} message", name, self.sent)
                };

                match tx.send(Chat::Message(msg)) {
                    Ok(_) => println!("{} sent message to {}", self.name, name),
                    Err(_) => println!("Failed to send")
                };
            }
        }
    }

    fn read(&mut self, chat: Chat) {
        let (format, from, contents) = match chat.clone() {
            Chat::Message(msg) => ("received a message", msg.from, msg.contents),
            Chat::Forward(fwd) => ("received a forwarded message", fwd.from, fwd.contents.contents)
        };

        println!("{} {} from {}. It read '{}'", self.name, format, from, contents);
        self.respond(chat);
    }

    pub fn respond(&mut self, chat: Chat) {
        if !self.is_active() {
            return self.logout();
        }

        self.sent += 1;
        let from = String::from(&self.name);

        let (to, message) = match chat {
            Chat::Message(msg) => (msg.from.clone(), Message::new(from.clone(), format!("Hi {}. This is {}", msg.from, from).as_str())),
            Chat::Forward(fwd) => (fwd.contents.from.clone(), Message::new(from, format!("{} asked me to forward this: {}", fwd.from, fwd.contents).as_str()))
        };

        match self.transmitters.get(to.as_str()) {
            Some(tx) => match tx.send(Chat::Message(message)) {
                Ok(_) => println!("{} sent message to {}", self.name, to),
                Err(_) => println!("Failed to send")
            },
            None => self.forward(message)
        };
    }

    pub fn start(&mut self) {
        match self.transmitters.keys().nth(0) {
            Some(to) => self.respond(Chat::Message(Message::new(to.to_string(), ""))),
            None => println!("{} has no one to talk to. =(", self.name)
        };

        while self.is_active() {
            match self.receiver.recv() {
                Ok(chat) => self.read(chat),
                Err(error) => println!("Error encountered: {}", error)
            };
        }

        self.logout();
    }
}