extern crate robus;

#[test]
fn main() {
    robus::init();

    let cb = |msg: robus::Message| {
        assert_eq!(msg.header.command, robus::Command::PublishState);
        assert_eq!(msg.data, vec![3, 2, 42]);
    };

    let mut core = robus::init();

    let module = core.create_module("fire_button", robus::ModuleType::Button, &cb);

    let command = robus::Command::PublishState;
    let data = vec![3, 2, 42];

    let mut sent_msg = robus::Message::broadcast(command, &data);
    core.send(module, &mut sent_msg);
}
