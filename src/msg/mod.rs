mod header;
use self::header::Header;

use Command;

const PROTOCOL_VERSION: u8 = 0;
const BROADCAST_TARGET: u16 = 0x0FFF;
const MAX_DATA_SIZE: usize = 256;

#[derive(Debug, PartialEq)]

pub struct Message {
    pub header: Header,
    pub data: Vec<u8>,
}

/// `TargetMode` is the message addressing mode enum.
/// This structure is used to get the message addressing mode list.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TargetMode {
    /// Unique or virtual ID, used to send something to only one module.
    Id = 0,
    /// Unique or virtual ID with reception Acknoledgment (ACK).
    IdAck,
    /// Type mode, used to send something to all module of the same type.
    Type,
    /// Broadcast mode, used to send something to everybody.
    Broadcast,
    /// Multicast mode, used to send something to multiple modules.
    Multicast,
}

impl Message {
    pub fn id(target: u16, command: Command, data: &Vec<u8>) -> Message {
        let size = data.len();
        Message {
            header: make_header(target, TargetMode::Id, command, size),
            data: data.clone(),
        }
    }
    pub fn idack(target: u16, command: Command, data: &Vec<u8>) -> Message {
        let size = data.len();
        Message {
            header: make_header(target, TargetMode::IdAck, command, size),
            data: data.clone(),
        }
    }
    pub fn type_msg(target: u16, command: Command, data: &Vec<u8>) -> Message {
        let size = data.len();
        Message {
            header: make_header(target, TargetMode::Type, command, size),
            data: data.clone(),
        }
    }
    pub fn broadcast(command: Command, data: &Vec<u8>) -> Message {
        let size = data.len();
        Message {
            header: make_header(BROADCAST_TARGET, TargetMode::Broadcast, command, size),
            data: data.clone(),
        }
    }
    pub fn multicast(target: u16, command: Command, data: &Vec<u8>) -> Message {
        let size = data.len();
        Message {
            header: make_header(target, TargetMode::Multicast, command, size),
            data: data.clone(),
        }
    }
    pub fn from_bytes(bytes: &[u8]) -> Message {
        let header = Header::from_bytes(
            &[bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]],
        );
        Message {
            data: bytes[header::HEADER_SIZE..header::HEADER_SIZE + header.data_size].to_vec(),
            header,
        }
    }
    pub fn to_bytes(&self) -> [u8; header::HEADER_SIZE + MAX_DATA_SIZE] {
        let raw_header = self.header.to_bytes();
        let mut unmap: [u8; header::HEADER_SIZE + MAX_DATA_SIZE] = [0;
            header::HEADER_SIZE + MAX_DATA_SIZE];
        let mut tmp = 0;
        for val in raw_header.iter() {
            unmap[tmp] = *val;
            tmp += 1;
        }
        for val in self.data.iter() {
            unmap[tmp] = *val;
            tmp += 1;
        }
        unmap
    }
}

fn make_header(target: u16, target_mode: TargetMode, command: Command, data_size: usize) -> Header {
    if data_size > MAX_DATA_SIZE {
        panic!("data_size over limits {}.", MAX_DATA_SIZE);
    }
    if target > 0b0000_1111_1111_1111 {
        panic!("target overflow.");
    }
    if target_mode as u8 > TargetMode::Multicast as u8 {
        panic!("target overflow.");
    }
    // TODO : we should add a panic for command too. To do that we could make a procedural macro
    //        that count the enum value number.
    Header {
        protocol: PROTOCOL_VERSION,
        target: target,
        target_mode: target_mode,
        source: 0,
        command: command,
        data_size: data_size,
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use super::header::tests::{rand_id, rand_command};

    #[test]
    fn create_id_message() {
        let target = rand_id();
        let command = rand_command();
        let data = rand_data();

        let msg = Message::id(target, command, &data);

        assert_eq!(msg.header.target, target);
        assert_eq!(msg.header.target_mode, TargetMode::Id);
        assert_eq!(msg.header.command, command);
        assert_eq!(msg.data, data);
    }

    #[test]
    fn create_broadcast() {
        let command = rand_command();
        let data = rand_data();

        let msg = Message::broadcast(command, &data);

        assert_eq!(msg.header.target, BROADCAST_TARGET);
        assert_eq!(msg.header.target_mode, TargetMode::Broadcast);
        assert_eq!(msg.header.command, command);
        assert_eq!(msg.data, data);
    }

    #[test]
    #[should_panic]
    fn invalid_target() {
        let invalid_target = rand_id() + 0b0000_1111_1111_1111;
        Message::id(invalid_target, rand_command(), &rand_data());
    }

    #[test]
    fn ser_deser() {
        let msg = rand_msg();
        assert_eq!(msg, Message::from_bytes(&msg.to_bytes()));
    }

    fn rand_data() -> Vec<u8> {
        vec![1, 2, 3, 4]
    }
    pub fn rand_msg() -> Message {
        Message::id(rand_id(), rand_command(), &rand_data())
    }
}
