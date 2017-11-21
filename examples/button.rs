extern crate robus;

use robus::{Command, Message};

const GATE_ID: u16 = 1;

fn main() {
    let (tx, rx) = robus::message_queue();

    let cb = move |msg: &Message| {
        let answer = match msg.header.command {
            Command::Identify => Some(Message::id(
                GATE_ID,
                Command::Introduction,
                &"hello".as_bytes().to_owned(), // TODO: read some real data here
            )),
            Command::GetState => Some(Message::id(
                GATE_ID,
                Command::PublishState,
                &vec![0], // TODO: read some real data here
            )),
            _ => None,
        };
        if let Some(answer) = answer {
            tx.send(answer);
        }
    };

    let mut core = robus::init();

    let button = core.create_module("fire_button", robus::ModuleType::Button, &cb);

    loop {
        if let Some(mut msg) = rx.recv() {
            core.send(&button, &mut msg);
        }
    }
}
