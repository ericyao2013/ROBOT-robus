use core::cell::UnsafeCell;
use alloc::rc::Rc;

use Message;
use super::deque::Deque;

/// Message queue for robus `Message`
///
/// Simplify the `Message` passing from the reception callback to the main loop where it can be send.
///
/// The queue only keeps a single message and is not thread or interrupt safe!
pub fn message_queue() -> (Tx, Rx) {
    let stack = Deque::new(1);
    let stack = Rc::new(UnsafeCell::new(stack));

    let tx = Tx { stack: stack.clone() };
    let rx = Rx { stack: stack.clone() };

    (tx, rx)
}

pub struct Tx {
    stack: Rc<UnsafeCell<Deque<Message>>>,
}
impl Tx {
    pub fn send(&self, msg: Message) {
        unsafe {
            (*self.stack.get()).push(msg);
        }
    }
}

pub struct Rx {
    stack: Rc<UnsafeCell<Deque<Message>>>,
}
impl Rx {
    pub fn recv(&self) -> Option<Message> {
        unsafe { (*self.stack.get()).pop() }
    }
}

#[cfg(test)]
pub mod tests {
    extern crate rand;

    use super::*;

    use self::rand::distributions::{IndependentSample, Range};
    use super::super::super::msg::tests::rand_msg;

    #[test]
    fn read_empty() {
        let (_, rx) = message_queue();

        assert_eq!(rx.recv(), None);
        // Check if still emtpy :)
        assert_eq!(rx.recv(), None);
    }
    #[test]
    fn send_and_read() {
        let (tx, rx) = message_queue();

        let send_msg = rand_msg();
        tx.send(send_msg.clone());

        let recv_msg = rx.recv().unwrap();
        assert_eq!(send_msg, recv_msg);

        assert_eq!(rx.recv(), None);
    }
    #[test]
    fn send_multiple() {
        let (tx, rx) = message_queue();

        let mut rng = rand::thread_rng();
        let n = Range::new(0, 42).ind_sample(&mut rng);

        for _ in 0..n {
            tx.send(rand_msg());
        }
        let send_msg = rand_msg();
        tx.send(send_msg.clone());

        let recv_msg = rx.recv().unwrap();
        assert_eq!(send_msg, recv_msg);

        assert_eq!(rx.recv(), None);

    }
}
