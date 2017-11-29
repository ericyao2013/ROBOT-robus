use msg::{Header, Message, CRC_SIZE, HEADER_SIZE};

static mut BUF: [u8; 300] = [0; 300];
static mut I: usize = 0;


pub struct RecvBuf {}

impl RecvBuf {
    // TODO: use a more dynamic version?
    pub fn with_capacity(_capacity: usize) -> RecvBuf {
        RecvBuf {}
    }
    pub fn push(&mut self, byte: u8) {
        unsafe {
            BUF[I] = byte;
            I += 1;
        }
    }
    pub fn get_message(&mut self) -> Option<Message> {
        unsafe {
            if I >= HEADER_SIZE {
                // TODO: stocker le header en static
                let header = Header::from_bytes(&BUF[..HEADER_SIZE]);

                if I == HEADER_SIZE + header.data_size + CRC_SIZE {
                    let msg = Message::from_bytes(&BUF[..I]);
                    I = 0;

                    return msg;
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
