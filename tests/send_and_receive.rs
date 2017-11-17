extern crate robus;

fn callback(msg: &robus::Message) {

    assert_eq!(msg.header.command, robus::Command::PublishState);
    assert_eq!(msg.data, vec![3, 2, 42]);
}

#[test]
fn main() {
    robus::init();

    let module = robus::Module::new("fire_button", robus::ModuleType::Button, callback);

    let command = robus::Command::PublishState;
    let data = vec![3, 2, 42];

    let mut sent_msg = robus::Message::broadcast(command, &data);
    module.send(&mut sent_msg);
}
