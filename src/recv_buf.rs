use msg::{Header, Message, CRC_SIZE, HEADER_SIZE};

const BUF_SIZE: usize = 300;
const MIN_MSG_SIZE: usize = HEADER_SIZE + CRC_SIZE;

static mut BUF: [u8; BUF_SIZE] = [0; BUF_SIZE];
static mut I: usize = 0;
static mut TO_READ: usize = MIN_MSG_SIZE;
static mut CRC: u16 = 0xFFFF;

pub fn flush() {
    unsafe {
        I = 0;
    }
}

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

            // An entire header has been received
            if I == HEADER_SIZE {
                match Header::from_bytes(&BUF[..HEADER_SIZE]) {
                    Ok(h) => {
                        TO_READ += h.data_size;
                    }
                    Err(_e) => self.flush(),
                }
            }

            // Update the computed CRC
            // unless we are actually reading the sent one.
            if I <= TO_READ - 2 {
                self.update_crc(byte);
            }
        }
    }
    fn flush(&mut self) {
        unsafe {
            I = 0;
            TO_READ = MIN_MSG_SIZE;
            CRC = 0xFFFF;
        }
    }
    pub fn get_message(&mut self) -> Option<Message> {
        if unsafe { I == TO_READ } {
            let msg = unsafe { Message::from_bytes(&BUF[..I], Some(CRC)) };
            self.flush();

            if msg.is_ok() {
                return Some(msg.unwrap());
            }
        }
        None
    }
    unsafe fn update_crc(&self, val: u8) {
        let mut crc = CRC;

        let mut x = (crc >> 8) as u8 ^ val;
        x ^= x >> 4;
        // TODO: use the proper CRC computation
        // This one is only kept for compatibility.
        crc = ((crc << 8) as u32 ^ (x as u32) << 12 ^ (x as u32) << 5 ^ x as u32) as u16;

        CRC = crc;
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
        let mut buf = RecvBuf::with_capacity(BUF_SIZE);
        buf.flush();

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
