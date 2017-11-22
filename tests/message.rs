extern crate robus;

use robus::{Command, Message};

#[test]
fn create_messages() {
    let target: u16 = 42;
    let data = vec![1, 2, 3, 4];

    let _msg = Message::id(target, Command::PublishState, &data);
    let _msg = Message::broadcast(Command::Identify, &data);
}
