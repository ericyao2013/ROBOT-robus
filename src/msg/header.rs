use super::{MAX_DATA_SIZE, PROTOCOL_VERSION, TargetMode};
use Command;
use std::mem;

#[derive(Debug, PartialEq)]
pub struct Header {
    pub protocol: u8,
    pub target: u16,
    pub target_mode: TargetMode,
    pub source: u16,
    pub command: Command,
    pub data_size: usize,
}

pub const HEADER_SIZE: usize = 6;

impl Header {
    pub fn from_bytes(bytes: &[u8; HEADER_SIZE]) -> Header {
        let header = Header {
            protocol: bytes[0] & 0b0000_1111,
            target: ((bytes[0] & 0b1111_0000) >> 4) as u16 | (bytes[1] as u16) << 4,
            target_mode: unsafe { mem::transmute::<u8, TargetMode>(bytes[2] & 0b0000_1111) },
            source: ((bytes[2] & 0b1111_0000) >> 4) as u16 | (bytes[3] as u16) << 4,
            command: unsafe { mem::transmute::<u8, Command>(bytes[4]) },
            data_size: bytes[5] as usize,
        };

        if header.data_size > MAX_DATA_SIZE {
            panic!("data_size over limits {}.", MAX_DATA_SIZE);
        }
        if header.protocol > PROTOCOL_VERSION {
            panic!("protocol version {} incompatible.", header.protocol);
        }
        if header.target_mode as u8 > TargetMode::Multicast as u8 {
            panic!("TargetMode out of range!");
        }
        // TODO : we should add a panic for command too. To do that we could make a procedural macro
        //        that count the enum value number.
        header
    }

    pub fn to_bytes(&self) -> [u8; HEADER_SIZE] {
        let mut unmap: [u8; HEADER_SIZE] = [0; HEADER_SIZE];
        unmap[0] = (unmap[0] & 0b1111_0000) | (self.protocol & 0b0000_1111);
        unmap[0] = (unmap[0] & 0b0000_1111) | ((self.target & 0b0000_0000_0000_1111) << 4) as u8;
        unmap[1] = (self.target >> 4) as u8;
        unmap[2] = self.target_mode as u8 & 0b0000_1111;
        unmap[2] = (unmap[2] & 0b0000_1111) | ((self.source & 0b0000_0000_0000_1111) << 4) as u8;
        unmap[3] = (self.source >> 4) as u8;
        unmap[4] = self.command as u8;
        unmap[5] = self.data_size as u8;
        unmap
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    extern crate rand;
    use self::rand::distributions::{IndependentSample, Range};

    #[test]
    fn create_header() {
        let target = rand_id();
        let target_mode = rand_target_mode();
        let source = rand_id();
        let command = rand_command();
        let data_size = rand_data_size();

        let header = Header {
            protocol: PROTOCOL_VERSION,
            target,
            target_mode,
            source,
            command,
            data_size,
        };

        assert_eq!(header.protocol, PROTOCOL_VERSION);
        assert_eq!(header.target, target);
        assert_eq!(header.target_mode, target_mode);
        assert_eq!(header.source, source);
        assert_eq!(header.command, command);
        assert_eq!(header.data_size, data_size);
    }

    #[test]
    fn raw_header() {
        let header = random_header();
        let unmap = header.to_bytes();

        assert_eq!(unmap.len(), HEADER_SIZE);

        assert_eq!(header.protocol, unmap[0] & 0b0000_1111);
        assert_eq!(
            header.target,
            (((unmap[0] & 0b1111_0000) >> 4) as u16 | (unmap[1] as u16) << 4)
        );
        assert_eq!(header.target_mode as u8, unmap[2] & 0b0000_1111);
        assert_eq!(
            header.source,
            (((unmap[2] & 0b1111_0000) >> 4) as u16 | (unmap[3] as u16) << 4)
        );
        assert_eq!(header.command as u8, unmap[4]);
        assert_eq!(header.data_size as u8, unmap[5]);
    }
    #[test]
    fn ser_deser() {
        let header = random_header();
        assert_eq!(header, Header::from_bytes(&header.to_bytes()));
    }

    fn random_header() -> Header {
        Header {
            protocol: PROTOCOL_VERSION,
            target: rand_id(),
            target_mode: rand_target_mode(),
            source: rand_id(),
            command: rand_command(),
            data_size: rand_data_size(),
        }
    }
    pub fn rand_id() -> u16 {
        let mut rng = rand::thread_rng();
        Range::new(0, 2u16.pow(12)).ind_sample(&mut rng)
    }
    pub fn rand_target_mode() -> TargetMode {
        TargetMode::Id
    }
    pub fn rand_command() -> Command {
        Command::PublishState
    }
    pub fn rand_data_size() -> usize {
        let mut rng = rand::thread_rng();
        Range::new(0, MAX_DATA_SIZE).ind_sample(&mut rng)
    }
}
