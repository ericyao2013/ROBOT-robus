use super::{MAX_DATA_SIZE, PROTOCOL_VERSION, TargetMode};
use Command;

#[derive(Debug, PartialEq)]
pub struct Header {
    protocol: u8,
    target: u16,
    target_mode: TargetMode,
    source: u16,
    command: Command,
    data_size: usize,
}

const HEADER_SIZE: usize = 6;

impl Header {
    pub fn from_raw(raw: [u8; HEADER_SIZE]) -> Header {
        Header {
            protocol: PROTOCOL_VERSION,
            target: 0,
            target_mode: TargetMode::Broadcast,
            source: 0,
            command: Command::GetPosition,
            data_size: 0,
        }
    }

    pub fn unmap(&self) -> [u8; HEADER_SIZE] {
        [0; HEADER_SIZE]
    }
}

#[cfg(test)]
mod tests {
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
        let unmap = header.unmap();

        assert_eq!(unmap.len(), HEADER_SIZE);

        assert_eq!(header.protocol, unmap[0] & 0b0000_1111);
        assert_eq!(
            header.target,
            (((unmap[0] & 0b1111_0000) as u16) << 8) + unmap[1] as u16
        );
        assert_eq!(header.target_mode as u8, unmap[2] & 0b0000_1111);
        assert_eq!(
            header.source,
            (((unmap[2] & 0b1111_0000) as u16) << 8) + unmap[3] as u16
        );
        assert_eq!(header.command as u8, unmap[4]);
        assert_eq!(header.data_size as u8, unmap[5]);
    }
    #[test]
    fn ser_deser() {
        let header = random_header();
        assert_eq!(header, Header::from_raw(header.unmap()));
    }

    #[test]
    #[should_panic]
    fn out_of_range_id() {
        let mut rng = rand::thread_rng();
        let offset = Range::new(1, 2u16.pow(4)).ind_sample(&mut rng);

        let invalid_target = rand_id() + offset;

        Header {
            protocol: PROTOCOL_VERSION,
            target: invalid_target,
            target_mode: rand_target_mode(),
            source: rand_id(),
            command: rand_command(),
            data_size: rand_data_size(),
        };
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
    fn rand_id() -> u16 {
        let mut rng = rand::thread_rng();
        Range::new(0, 2u16.pow(12)).ind_sample(&mut rng)
    }
    fn rand_target_mode() -> TargetMode {
        TargetMode::Id
    }
    fn rand_command() -> Command {
        Command::Publish
    }
    fn rand_data_size() -> usize {
        let mut rng = rand::thread_rng();
        Range::new(0, MAX_DATA_SIZE).ind_sample(&mut rng)
    }
}
