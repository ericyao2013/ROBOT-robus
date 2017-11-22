/// Available Command for `Message`
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    Identify,
    Introduction,

    GetState,
    PublishState,
}
