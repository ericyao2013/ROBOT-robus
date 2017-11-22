extern crate robus;

use robus::{Command, Message};

const GATE_ID: u16 = 1;

fn main() {
    robus::init();

    let (tx, rx) = robus::message_queue();

    let button = robus::Module::new("fire_button", robus::ModuleType::Button, move |msg| {
        let answer = match msg.header.command {
            Command::Identify => Some(Message::id(
                GATE_ID,
                Command::Introduction,
                &"hello".as_bytes().to_owned(),
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
    });

    loop {
        if let Some(mut msg) = rx.recv() {
            button.send(&mut msg);
        }
    }
}
