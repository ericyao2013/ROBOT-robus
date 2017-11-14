mod header;
use self::header::Header;

use Command;

const PROTOCOL_VERSION: u8 = 0;
const BROADCAST_TARGET: u16 = 0;
const MAX_DATA_SIZE: usize = 256;

#[derive(Debug, PartialEq)]
pub struct Message<'a> {
    header: Header,
    data: &'a[u8],
}

impl<'a> Message<'a> {
    pub fn id(target: u16, command: Command, data: &[u8]) -> Message {
        Message {
            header: make_header(),
            data,
        }
    }
    pub fn broadcast(command: Command, data: &[u8]) -> Message {
        Message {
            header: make_header(),
            data,
        }
    }
    pub fn from_bytes(bytes: &[u8]) -> Message {
        Message {
            header: make_header(),
            data: &[1, 2, 3],
        }
    }
    pub fn to_bytes(&self) -> &'a[u8] {
        &[1, 2, 3]
    }
}

fn make_header() -> Header {
    Header {
        protocol: 0,
        target: 0,
        target_mode: TargetMode::Id,
        source: 0,
        command: Command::GetState,
        data_size: 0,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TargetMode {
    Broadcast,
    Id,
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate rand;
    use self::rand::distributions::{IndependentSample, Range};

    use super::header::tests::{rand_id, rand_command};

    #[test]
    fn create_id_message() {
        let target = rand_id();
        let command = rand_command();
        let data = rand_data();

        let msg = Message::id(target, command, data);

        assert_eq!(msg.header.target, target);
        assert_eq!(msg.header.target_mode, TargetMode::Id);
        assert_eq!(msg.header.command, command);
        assert_eq!(msg.data, data);
    }

    #[test]
    fn create_broadcast() {
        let command = rand_command();
        let data = rand_data();

        let msg = Message::broadcast(command, data);

        assert_eq!(msg.header.target, BROADCAST_TARGET);
        assert_eq!(msg.header.target_mode, TargetMode::Broadcast);
        assert_eq!(msg.header.command, command);
        assert_eq!(msg.data, data);
    }

    #[test]
    #[should_panic]
    fn invalid_target() {
        let mut rng = rand::thread_rng();
        let offset = Range::new(1, 2u16.pow(4)).ind_sample(&mut rng);
        let invalid_target = rand_id() + offset;

        Message::id(invalid_target, rand_command(), rand_data());
    }

    #[test]
    fn ser_deser() {
        let msg = rand_msg();
        assert_eq!(msg, Message::from_bytes(msg.to_bytes()));
    }

    fn rand_data<'a>() -> &'a[u8] {
        &[1, 2, 3, 4]
    }
    fn rand_msg<'a>() -> Message<'a> {
        Message::id(rand_id(), rand_command(), rand_data())
    }
}
