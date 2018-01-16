extern crate robus;

const BAUDRATE: u32 = 57600;

#[test]
fn main() {
    let mut core = robus::init(BAUDRATE);

    let cb = |msg: robus::Message| {
        assert_eq!(msg.header.command, robus::Command::PublishState);
        assert_eq!(msg.data, vec![3, 2, 42]);
    };

    let module = core.create_module("fire_button", robus::ModuleType::Button, &cb);

    let command = robus::Command::PublishState;
    let data = vec![3, 2, 42];

    let mut sent_msg = robus::Message::broadcast(command, &data);
    core.send(module, &mut sent_msg);
}
