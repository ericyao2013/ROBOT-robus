use msg::{Header, Message, CRC_SIZE, HEADER_SIZE};

const BUF_SIZE: usize = 300;
const MIN_MSG_SIZE: usize = HEADER_SIZE + CRC_SIZE;

static mut BUF: [u8; BUF_SIZE] = [0; BUF_SIZE];
static mut I: usize = 0;
static mut TO_READ: usize = MIN_MSG_SIZE;
static mut CRC: u16 = 0xFFFF;

pub fn push(byte: u8) {
    unsafe {
        BUF[I] = byte;
        I += 1;

        // An entire header has been received
        if I == HEADER_SIZE {
            match Header::from_bytes(&BUF[..HEADER_SIZE]) {
                Ok(h) => {
                    TO_READ += h.data_size;
                }
                Err(_e) => flush(),
            }
        }

        // Update the computed CRC
        // unless we are actually reading the sent one.
        if I <= TO_READ - 2 {
            update_crc(byte);
        }
    }
}
pub fn flush() {
    unsafe {
        I = 0;
        TO_READ = MIN_MSG_SIZE;
        CRC = 0xFFFF;
    }
}
pub fn get_message() -> Option<Message> {
    if unsafe { I == TO_READ } {
        let msg = unsafe { Message::from_bytes(&BUF[..I], Some(CRC)) };
        flush();

        if msg.is_ok() {
            return Some(msg.unwrap());
        }
    }
    None
}
unsafe fn update_crc(val: u8) {
    let mut crc = CRC;

    let mut x = (crc >> 8) as u8 ^ val;
    x ^= x >> 4;
    // TODO: use the proper CRC computation
    // This one is only kept for compatibility.
    crc = ((crc << 8) as u32 ^ (x as u32) << 12 ^ (x as u32) << 5 ^ x as u32) as u16;

    CRC = crc;
}

#[cfg(test)]
mod tests {
    use recv_buf;
    use msg::tests::rand_msg;

    extern crate rand;
    use self::rand::distributions::{IndependentSample, Range};

    #[test]
    fn parse() {
        recv_buf::flush();

        let mut rng = rand::thread_rng();
        let n = Range::new(1, 10).ind_sample(&mut rng);

        for _ in 0..n {
            let msg = rand_msg();
            let bytes = msg.to_bytes();

            for d in bytes[..bytes.len() - 1].iter() {
                recv_buf::push(*d);
                assert_eq!(recv_buf::get_message(), None);
            }
            recv_buf::push(bytes[bytes.len() - 1]);
            assert_eq!(recv_buf::get_message(), Some(msg));
        }
    }
}
