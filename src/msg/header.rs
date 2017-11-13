use super::{MAX_DATA_SIZE, PROTOCOL_VERSION, TargetMode};
use Command;

const HEADER_SIZE: usize = 6;
pub struct Header {
    unmap: [u8; HEADER_SIZE],
}

impl Header {
    pub fn new(
        _target: u16,
        _target_mode: TargetMode,
        _source: u16,
        _command: Command,
        _data_size: usize,
    ) -> Header {
        Header { unmap: [0; HEADER_SIZE] }
    }
    pub fn protocol(&self) -> u8 {
        0
    }
    pub fn target(&self) -> u16 {
        0
    }
    pub fn target_mode(&self) -> TargetMode {
        TargetMode::Broadcast
    }
    pub fn source(&self) -> u16 {
        0
    }
    pub fn command(&self) -> Command {
        Command::GetPosition
    }
    pub fn data_size(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate rand;
    use self::rand::distributions::{IndependentSample, Range};

    #[test]
    fn create_header() {
        let (target, target_mode, source, command, data_size) = rand_header_data();
        let header = Header::new(target, target_mode, source, command, data_size);

        assert_eq!(header.protocol(), PROTOCOL_VERSION);
        assert_eq!(header.target(), target);
        assert_eq!(header.target_mode(), target_mode);
        assert_eq!(header.source(), source);
        assert_eq!(header.command(), command);
        assert_eq!(header.data_size(), data_size);
    }

    #[test]
    fn raw_header() {
        let (target, target_mode, source, command, data_size) = rand_header_data();
        let header = Header::new(target, target_mode, source, command, data_size);

        assert_eq!(header.unmap.len(), HEADER_SIZE);

        assert_eq!(PROTOCOL_VERSION, header.unmap[0] & 0b0000_1111);
        assert_eq!(
            target,
            (((header.unmap[0] & 0b1111_0000) as u16) << 8) + header.unmap[1] as u16
        );
        assert_eq!(target_mode as u8, header.unmap[2] & 0b0000_1111);
        assert_eq!(
            source,
            (((header.unmap[2] & 0b1111_0000) as u16) << 8) + header.unmap[3] as u16
        );
        assert_eq!(command as u8, header.unmap[4]);
        assert_eq!(data_size as u8, header.unmap[5]);
    }
    #[test]
    #[should_panic]
    fn out_of_range_id() {
        let mut rng = rand::thread_rng();
        let offset = Range::new(1, 2u16.pow(4)).ind_sample(&mut rng);

        let invalid_target = rand_id() + offset;

        let (_, target_mode, source, command, data_size) = rand_header_data();
        Header::new(invalid_target, target_mode, source, command, data_size);
    }

    fn rand_header_data() -> (u16, TargetMode, u16, Command, usize) {
        let target = rand_id();
        let target_mode = rand_target_mode();
        let source = rand_id();
        let command = rand_command();
        let data_size = rand_data_size();

        (target, target_mode, source, command, data_size)
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
