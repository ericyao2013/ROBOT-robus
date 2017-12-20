use alloc::String;

use {error, Command};
use super::TargetMode;

#[derive(Debug, PartialEq)]
pub enum ParsingError {
    InvalidCommand(Command),
    InvalidCrc((u16, u16)),
    InvalidDataSize(usize),
    InvalidHeaderSize(usize),
    InvalidProtocol(u8),
    InvalidTargetMode(TargetMode),
}

impl error::Error for ParsingError {
    fn description(&self) -> String {
        match *self {
            ParsingError::InvalidCommand(c) => format!("Invalid Command {:?}", c),
            ParsingError::InvalidCrc((c1, c2)) => format!("Invalid CRC ({} vs {})", c1, c2),
            ParsingError::InvalidDataSize(s) => format!("Invalid data size: {}", s),
            ParsingError::InvalidHeaderSize(l) => format!("Invalid header size: {:?}", l),
            ParsingError::InvalidProtocol(p) => format!("Invalid protocol {:?}", p),
            ParsingError::InvalidTargetMode(t) => format!("Invalid target mode {:?}", t),
        }
    }
}
