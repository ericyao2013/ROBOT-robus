extern crate robus;

#[test]
fn main() {
    robus::init();

    let module = robus::Module::new("fire_button", robus::ModuleType::Button);

    let command = robus::Command::PublishState;
    let data = vec![3, 2, 42];

    let sent_msg = robus::Message::broadcast(command, &data);
    module.send(&sent_msg);

    let recv_msg = module.read();
    assert!(recv_msg.is_some());

    let recv_msg = recv_msg.unwrap();
    assert_eq!(recv_msg.header.command, command);
    assert_eq!(recv_msg.data, data);
}
