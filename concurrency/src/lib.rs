mod chat;
mod exists_within;
mod forward;
mod message;
mod user;

use std::collections::HashMap;
use std::sync::mpsc::{sync_channel,SyncSender};
use std::thread;
use std::thread::JoinHandle;

use rand::prelude::*;

use chat::Chat;
use exists_within::ExistsWithin;
use user::User;

struct Usage<T> {
    count: u8,
    value: T
}

impl <T> Usage<T> where T: Clone {
    pub fn new(value: T) -> Self {
        Usage { value, count: 0 }
    }

    pub fn incr(&mut self) -> T {
        self.count += 1;
        self.value.clone()
    }
}

pub fn main(args: &[String]) {
    let mut channels: HashMap<String, Usage<SyncSender<Chat>>> = HashMap::new();
    let mut handles: Vec<JoinHandle<_>> = Vec::new();
    let mut rng = thread_rng();
    let mut users: Vec<User> = Vec::new();

    for arg in args {
        let (tx, rx) = sync_channel::<Chat>(1);
        let limit = (random::<u8>() % 10) + 2;
        let user = User::new(arg.to_string(), limit, rx);
        users.push(user);
        channels.insert(arg.to_string(), Usage::new(tx));
    }

    loop {
        if !channels.values().exists_within(|usage| usage.count == 0) {
            break;
        }

        let i = (0..users.len()).choose(&mut rng).unwrap();
        let mut user = users.remove(i);
        let to = channels.keys().choose(&mut rng).unwrap().clone();
        let mut tx = channels.remove(&to).unwrap();
        tx.incr();
        user.add(to.to_string(), tx.incr());

        channels.insert(to, tx);
        users.push(user);
    }

    for mut user in users {
        let handle = thread::spawn(move || {
            user.start();
        });

        handles.push(handle);
    }

    for handle in handles {
        match handle.join() {
            Ok(value) => println!("{:?}", value),
            Err(_) => println!("An error was encountered")
        }
    }
}