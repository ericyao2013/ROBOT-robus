#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    Identify,
    Publish,

    GetPosition,
}
