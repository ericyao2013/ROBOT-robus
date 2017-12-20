use alloc::String;

pub trait Error {
    fn description(&self) -> String;
}
