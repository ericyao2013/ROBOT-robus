use alloc::vec::Vec;

mod header;
pub use self::header::{Header, HEADER_SIZE, TargetMode};

use Command;

/// Current protocol revision.
const PROTOCOL_VERSION: u8 = 0;
/// Specific target value used on Broadcast `TargetMode`.
const BROADCAST_TARGET: u16 = 0x0FFF;
/// Max size of the data vector.
const MAX_DATA_SIZE: usize = 256;
// CRC size
pub const CRC_SIZE: usize = 2;
// Max size of a message.
pub const MAX_MESSAGE_SIZE: usize = HEADER_SIZE + MAX_DATA_SIZE + CRC_SIZE;

#[derive(Clone, Debug, PartialEq)]
/// Robus Message struct used for sending and receving
///
/// ## Examples
/// ```
/// use robus::{Command, Message};
///
/// let id_msg = Message::id(1, Command::PublishState, &vec![42]);
/// let broadcast_msg = Message::broadcast(Command::Identify, &vec![]);
/// ```
pub struct Message {
    /// Contain the message context allowing Robus to interpreat the data field.
    pub header: Header,
    /// The core data of the message.
    pub data: Vec<u8>,
}

impl Message {
    /// Returns a pre-filled `TargetMode::Id` message used to send data to only one module.
    ///
    /// # Arguments
    ///
    /// * `target` - A u16 designating the id of the target (max value is actually a u12).
    /// * `command` - A `Command` struct designating the purpose of the message.
    /// * `data` - A `&Vec<u8>` containing the data to transmit.
    pub fn id(target: u16, command: Command, data: &Vec<u8>) -> Message {
        Message::new(target, TargetMode::Id, command, data)
    }
    /// Returns a pre-filled `TargetMode::IdAck` message used to send
    /// data to only one module and get an Acknowledgment (ACK) back.
    ///
    /// # Arguments
    ///
    /// * `target` - A u16 designating the id of the target (max value is actually a u12).
    /// * `command` - A `Command` struct designating the purpose of the message.
    /// * `data` - A `&Vec<u8>` containing the data to transmit.
    pub fn id_ack(target: u16, command: Command, data: &Vec<u8>) -> Message {
        Message::new(target, TargetMode::IdAck, command, data)
    }
    /// Returns a pre-filled `TargetMode::Type` message used to send data
    /// to all modules of a same type.
    ///
    /// # Arguments
    ///
    /// * `target` - A u16 designating the `Type` of the targets (max value is actually a u12).
    /// * `command` - A `Command` struct designating the purpose of the message.
    /// * `data` - A `&Vec<u8>` containing the data to transmit.
    pub fn type_msg(target: u16, command: Command, data: &Vec<u8>) -> Message {
        Message::new(target, TargetMode::Type, command, data)
    }
    /// Returns a pre-filled `TargetMode::Broadcast` message used to send data to everybody.
    ///
    /// # Arguments
    ///
    /// * `command` - A `Command` struct designating the purpose of the message.
    /// * `data` - A `&Vec<u8>` containing the data to transmit.
    pub fn broadcast(command: Command, data: &Vec<u8>) -> Message {
        Message::new(BROADCAST_TARGET, TargetMode::Broadcast, command, data)
    }
    /// Returns a pre-filled `TargetMode::Multicast` message used to send data to a group of pre-registred modules.
    ///
    /// # Arguments
    ///
    /// * `target` - A u16 designating the Multicast id of the group targets (max value is actually a u12).
    /// * `command` - A `Command` struct designating the purpose of the message.
    /// * `data` - A `&Vec<u8>` containing the data to transmit.
    pub fn multicast(target: u16, command: Command, data: &Vec<u8>) -> Message {
        Message::new(target, TargetMode::Multicast, command, data)
    }
    /// Returns a message struct
    ///
    /// # Arguments
    ///
    /// * `target` - A u16 designating the target(s) (max value is u12)
    /// * `target_mode` - A TargetMode struct designating the targetting mode of the message
    /// * `command` - A Command struct designating the purpose of the message.
    /// * `data` - A &Vec\<u8\> containing data to trasmit.
    fn new(target: u16, target_mode: TargetMode, command: Command, data: &Vec<u8>) -> Message {
        Message {
            header: Header {
                protocol: PROTOCOL_VERSION,
                target: target,
                target_mode: target_mode,
                source: 0,
                command: command,
                data_size: data.len(),
            },
            data: data.clone(),
        }
    }
    /// Returns a Option<Message> struct from raw bytes.
    ///
    /// The construction can fail if the crc is not valid.
    ///
    /// # Argument
    ///
    /// * `bytes` - An `&Vec<u8> array of unmapped message data
    pub fn from_bytes(bytes: &[u8]) -> Option<Message> {
        let header = Header::from_bytes(&bytes[..HEADER_SIZE]);
        let data = bytes[HEADER_SIZE..(HEADER_SIZE + header.data_size)].to_vec();

        let calc_crc: u16 = crc(&bytes[..(HEADER_SIZE + header.data_size)]);
        let crc: u16 = (bytes[(HEADER_SIZE + header.data_size)] as u16) |
            ((bytes[(HEADER_SIZE + header.data_size + 1)] as u16) << 8);

        if calc_crc == crc {
            Some(Message { header, data })
        } else {
            None
        }
    }
    /// Returns raw bytes from a Message struct.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut unmap = self.header.to_bytes().to_vec();
        unmap.extend_from_slice(&self.data);
        let crc = crc(&unmap);
        unmap.extend_from_slice(&[crc as u8, (crc >> 8) as u8]);
        unmap
    }
}

fn crc(bytes: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    let mut x: u8;
    for val in bytes {
        x = (crc >> 8) as u8 ^ val;
        x ^= x >> 4;
        // TODO: use the proper CRC computation
        // This one is only kept for compatibility.
        crc = ((crc << 8) as u32 ^ (x as u32) << 12 ^ (x as u32) << 5 ^ x as u32) as u16;
    }
    crc
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

        assert_eq!(msg, Message::from_bytes(&msg.to_bytes()).unwrap());
    }
    #[test]
    fn check_crc() {
        let b1 = [48, 0, 32, 0, 33, 1, 0];
        let crc1 = [48, 34];
        assert_eq!(crc(&b1), (crc1[0] as u16) | ((crc1[1] as u16) << 8));

        let b2 = [48, 0, 32, 0, 33, 1, 1];
        let crc2 = [17, 50];
        assert_eq!(crc(&b2), (crc2[0] as u16) | ((crc2[1] as u16) << 8));
    }
    pub fn rand_data(size: usize) -> Vec<u8> {
        assert!(size < MAX_DATA_SIZE);
        let mut data = Vec::new();
        for _ in 0..size {
            data.push(rand::random::<u8>());
        }
        data
    }
    pub fn rand_msg() -> Message {
        Message::new(
            rand_id(),
            rand_target_mode(),
            rand_command(),
            &rand_data(rand_data_size()),
        )
    }
}
