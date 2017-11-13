mod header;
use self::header::Header;

const PROTOCOL_VERSION: u8 = 0;
const MAX_DATA_SIZE: usize = 256;

pub struct Message {
    _header: Header,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TargetMode {
    Broadcast,
    Id,
}
