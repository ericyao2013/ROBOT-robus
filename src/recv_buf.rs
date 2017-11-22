use alloc::vec::Vec;

use Message;

use msg::{Header, CRC_SIZE, HEADER_SIZE};

pub struct RecvBuf {
    buf: Vec<u8>,
}

impl RecvBuf {
    pub fn with_capacity(capacity: usize) -> RecvBuf {
        RecvBuf { buf: Vec::with_capacity(capacity) }
    }
    pub fn push(&mut self, byte: u8) {
        self.buf.push(byte);
    }
    pub fn get_message(&mut self) -> Option<Message> {
        let buf_len = self.buf.len();
        if buf_len >= HEADER_SIZE {
            let header = Header::from_bytes(&self.buf[..HEADER_SIZE]);

            if buf_len == HEADER_SIZE + header.data_size + CRC_SIZE {
                let msg = Message::from_bytes(&self.buf);
                self.buf.clear();

                if let Some(msg) = msg {
                    return Some(msg);
                } else {
                    // TODO: we should probably add some warning here
                    // to let users know that a corrupted message was received.
                    return None;
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use msg::tests::rand_msg;

    extern crate rand;
    use self::rand::distributions::{IndependentSample, Range};

    #[test]
    fn parse() {
        let mut buf = RecvBuf::with_capacity(100);

        let mut rng = rand::thread_rng();
        let n = Range::new(1, 10).ind_sample(&mut rng);

        for _ in 0..n {
            let msg = rand_msg();
            let bytes = msg.to_bytes();

            for d in bytes[..bytes.len() - 1].iter() {
                buf.push(*d);
                assert_eq!(buf.get_message(), None);
            }
            buf.push(bytes[bytes.len() - 1]);
            assert_eq!(buf.get_message(), Some(msg));
        }
    }
}
