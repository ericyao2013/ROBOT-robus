mod header;
use self::header::{Header, TargetMode, HEADER_SIZE};

use Command;

/// Current protocol revision.
const PROTOCOL_VERSION: u8 = 0;
/// Specific target value used on Broadcast `TargetMode`.
const BROADCAST_TARGET: u16 = 0x0FFF;
/// Max size of the data vector.
const MAX_DATA_SIZE: usize = 256;

#[derive(Debug, PartialEq)]
/// message struct
pub struct Message {
    /// Contain the message context allowing Robus to interpreat the data field.
    pub header: Header,
    /// The core datas of the message.
    pub data: Vec<u8>,
}

impl Message {
    /// Returns a pre-filled `TargetMode` Id message used to send something to only one module.
    ///
    /// # Arguments
    ///
    /// * `target` - A u16 designating the Id of the target (max value is u12)
    /// * `command` - A Command struct designating the purpose of the message.
    /// * `data` - A &Vec\<u8\> containing data to trasmit.
    pub fn id(target: u16, command: Command, data: &Vec<u8>) -> Message {
        make_message(target, TargetMode::Id, command, data)
    }
    /// Returns a pre-filled `TargetMode` IdAck message used to send
    /// something to only one module and get back an Acknoledgment (ACK).
    ///
    /// # Arguments
    ///
    /// * `target` - A u16 designating the Id of the target (max value is u12)
    /// * `command` - A Command struct designating the purpose of the message.
    /// * `data` - A &Vec\<u8\> containing data to trasmit.
    pub fn id_ack(target: u16, command: Command, data: &Vec<u8>) -> Message {
        make_message(target, TargetMode::IdAck, command, data)
    }
    /// Returns a pre-filled `TargetMode` Type message used to send something
    /// to all module of the same type.
    ///
    /// # Arguments
    ///
    /// * `target` - A u16 designating the Type of the targets (max value is u12)
    /// * `command` - A Command struct designating the purpose of the message.
    /// * `data` - A &Vec\<u8\> containing data to trasmit.
    pub fn type_msg(target: u16, command: Command, data: &Vec<u8>) -> Message {
        make_message(target, TargetMode::Type, command, data)
    }
    /// Returns a pre-filled `TargetMode` Broadcast message used to send something to everybody.
    ///
    /// # Arguments
    ///
    /// * `command` - A Command struct designating the purpose of the message.
    /// * `data` - A &Vec\<u8\> containing data to trasmit.
    pub fn broadcast(command: Command, data: &Vec<u8>) -> Message {
        make_message(BROADCAST_TARGET, TargetMode::Broadcast, command, data)
    }
    /// Returns a pre-filled `TargetMode` Multicast message used to send something to multiple modules.
    ///
    /// # Arguments
    ///
    /// * `target` - A u16 designating the Multicast id of targets (max value is u12)
    /// * `command` - A Command struct designating the purpose of the message.
    /// * `data` - A &Vec\<u8\> containing data to trasmit.
    pub fn multicast(target: u16, command: Command, data: &Vec<u8>) -> Message {
        make_message(target, TargetMode::Multicast, command, data)
    }
    /// Returns a message struct from raw datas
    ///
    /// # Argument
    ///
    /// * `bytes` - A [u8] array of unmapped message data
    pub fn from_bytes(bytes: &[u8]) -> Message {
        let header = Header::from_bytes(
            &[bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]],
        );
        Message {
            data: bytes[HEADER_SIZE..(HEADER_SIZE + header.data_size)].to_vec(),
            header,
        }
    }
    /// Returns raw datas from a message struct
    pub fn to_bytes(&self) -> [u8; HEADER_SIZE + MAX_DATA_SIZE] {
        let raw_header = self.header.to_bytes();
        let mut unmap: [u8; HEADER_SIZE + MAX_DATA_SIZE] = [0; HEADER_SIZE + MAX_DATA_SIZE];
        for (i, val) in raw_header.iter().enumerate() {
            unmap[i] = *val;
        }
        for (i, val) in self.data.iter().enumerate() {
            unmap[i + HEADER_SIZE] = *val;
        }
        unmap
    }
}

/// Returns a message struct
///
/// # Arguments
///
/// * `target` - A u16 designating the target(s) (max value is u12)
/// * `target_mode` - A TargetMode struct designating the targetting mode of the message
/// * `command` - A Command struct designating the purpose of the message.
/// * `data` - A &Vec\<u8\> containing data to trasmit.
fn make_message(target: u16, target_mode: TargetMode, command: Command, data: &Vec<u8>) -> Message {
    let header = Header {
        protocol: PROTOCOL_VERSION,
        target: target,
        target_mode: target_mode,
        source: 0,
        command: command,
        data_size: data.len(),
    };
    Message {
        header,
        data: data.clone(),
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    extern crate rand;

    pub use super::header::tests::{rand_id, rand_command, rand_data_size, rand_target_mode};

    #[test]
    fn create_id_message() {
        let target = rand_id();
        let command = rand_command();
        let data = rand_data(rand_data_size());

        let msg = Message::id(target, command, &data);

        assert_eq!(msg.header.target, target);
        assert_eq!(msg.header.target_mode, TargetMode::Id);
        assert_eq!(msg.header.command, command);
        assert_eq!(msg.data, data);
    }

    #[test]
    fn create_broadcast() {
        let command = rand_command();
        let data = rand_data(rand_data_size());

        let msg = Message::broadcast(command, &data);

        assert_eq!(msg.header.target, BROADCAST_TARGET);
        assert_eq!(msg.header.target_mode, TargetMode::Broadcast);
        assert_eq!(msg.header.command, command);
        assert_eq!(msg.data, data);
    }

    #[test]
    #[should_panic]
    fn invalid_target() {
        let invalid_target = rand_id() + header::MAX_ID_VAL;
        let msg = Message::id(invalid_target, rand_command(), &rand_data(rand_data_size()));
        msg.to_bytes();
    }

    #[test]
    fn ser_deser() {
        let msg = rand_msg();
        assert_eq!(msg, Message::from_bytes(&msg.to_bytes()));
    }

    fn rand_data(size: usize) -> Vec<u8> {
        assert!(size < MAX_DATA_SIZE);
        let mut data = Vec::new();
        for _ in 0..size {
            data.push(rand::random::<u8>());
        }
        data
    }
    pub fn rand_msg() -> Message {
        make_message(
            rand_id(),
            rand_target_mode(),
            rand_command(),
            &rand_data(rand_data_size()),
        )
    }
}
