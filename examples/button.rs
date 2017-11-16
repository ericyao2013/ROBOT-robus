extern crate robus;

use robus::{Command, Message};

const GATE_ID: u16 = 1;

fn main() {
    robus::init();

    let mut button = robus::Module::new(
        "fire_button",
        robus::ModuleType::Button,
        Box::new(|msg| {
            let answer = match msg.header.command {
                Command::Identify => Some(Message::id(
                    GATE_ID,
                    Command::Introduction,
                    &"plop".as_bytes().to_owned(),
                )),
                Command::GetState => Some(Message::id(
                    GATE_ID,
                    Command::PublishState,
                    &vec![0], // TODO: read some real data here
                )),
                _ => None,
            };
            if let Some(mut answer) = answer {
                // button.send(&mut answer);
            }
        }),
    );

    loop {}
}
