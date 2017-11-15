extern crate robus;

use robus::{Command, Message};

const GATE_ID: u16 = 1;

fn main() {
    robus::init();

    let button = robus::Module::new("fire_button", robus::ModuleType::Button);

    button.set_cb(|msg| {
        let answer = match msg.header.command {
            Command::Identify => Some(Message::id(
                GATE_ID,
                Command::Introduction,
                &button.alias.as_bytes().to_owned(),
            )),
            Command::GetState => Some(Message::id(
                GATE_ID,
                Command::PublishState,
                &vec![0], // TODO: read some real data here
            )),
            _ => None,
        };
        if let Some(answer) = answer {
            button.send(&answer);
        }
    });

    loop {}
}
