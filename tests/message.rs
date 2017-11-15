extern crate robus;

use robus::{Command, Message};

#[test]
fn create_messages() {
    let target: u16 = 42;
    let data = vec![1, 2, 3, 4];

    let msg = Message::id(target, Command::PublishState, &data);
    let msg = Message::broadcast(Command::Identify, &data);
}
