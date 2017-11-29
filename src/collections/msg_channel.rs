use core::cell::RefCell;
use alloc::rc::Rc;

use lock;

use Message;
use super::deque::Deque;

/// Message queue for robus `Message`
///
/// Simplify the `Message` passing from the reception callback to the main loop where it can be send.
///
/// The queue only keeps a single message!
pub fn message_queue() -> (Tx, Rx) {
    let stack = Deque::new(1);
    let stack_ref = Rc::new(RefCell::new(stack));

    let tx = Tx { stack_ref: stack_ref.clone() };
    let rx = Rx { stack_ref: stack_ref.clone() };

    (tx, rx)
}

pub struct Tx {
    stack_ref: Rc<RefCell<Deque<Message>>>,
}
impl Tx {
    pub fn send(&self, msg: Message) {
        let mut stack = self.stack_ref.borrow_mut();
        stack.push(msg);
    }
}

pub struct Rx {
    stack_ref: Rc<RefCell<Deque<Message>>>,
}
impl Rx {
    pub fn recv(&self) -> Option<Message> {
        lock::take();
        let msg = self.stack_ref.borrow_mut().pop();
        lock::release();

        msg
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
    #[test]
    fn multiple() {}
}
