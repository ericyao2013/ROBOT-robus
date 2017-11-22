use alloc::rc::Rc;
use core::cell::RefCell;

use {Message, Module, ModuleType};

use msg::MAX_MESSAGE_SIZE;
use registry::Registry;
use recv_buf::RecvBuf;

pub struct Core<'a> {
    registry: Registry<'a>,
    recv_buf: RecvBuf,
}

impl<'a> Core<'a> {
    pub fn new() -> Core<'a> {
        Core {
            registry: Registry::new(),
            recv_buf: RecvBuf::with_capacity(MAX_MESSAGE_SIZE),
        }
    }
    pub fn create_module(
        &mut self,
        alias: &'a str,
        mod_type: ModuleType,
        cb: &'a Fn(&Message),
    ) -> Rc<RefCell<Module<'a>>> {

        let module = Module::new(alias, mod_type, cb);
        let mod_ref = Rc::new(RefCell::new(module));

        self.registry.add(mod_ref.clone());

        mod_ref
    }
    pub fn receive(&mut self, byte: u8) {
        self.recv_buf.push(byte);

        if let Some(msg) = self.recv_buf.get_message() {
            let matches = self.registry.find_targeted_modules(&msg);

            for mod_ref in matches.iter() {
                let module = mod_ref.borrow();
                (module.callback)(&msg);
            }
        }
    }
    pub fn send(&mut self, from: &Rc<RefCell<Module>>, mut msg: &mut Message) {
        let module = from.borrow();
        module.send(&mut msg);

        for byte in msg.to_bytes() {
            self.receive(byte);
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;

    use self::std::time;

    use module::tests::rand_type;
    use msg::tests::{rand_command, rand_data, rand_data_size, rand_id};

    macro_rules! wait_timeout {
        ($evt: expr, $dur: expr, $cb: expr) => (
            let now = time::SystemTime::now();
            while !$evt.is_set() {
                let dt = now.elapsed().unwrap();
                if dt > $dur {
                    $cb();
                    break;
                }
            }
        );
    }

    #[test]
    fn id_local_loop() {
        let mut send_msg = rand_id_msg();
        let gold_msg = send_msg.clone();

        let (called_tx, called_rx) = Event::new();

        let m1_cb = move |msg: &Message| {
            assert_eq!(msg.header.command, gold_msg.header.command);
            assert_eq!(msg.data, gold_msg.data);
            called_tx.set();
        };
        let m2_cb = move |_msg: &Message| {
            assert!(false);
        };

        let mut core = Core::new();

        let m1 = core.create_module("m1", rand_type(), &m1_cb);
        m1.borrow_mut().id = send_msg.header.target;

        let mut diff_id = rand_id();
        while diff_id == send_msg.header.target {
            diff_id = rand_id();
        }
        let m2 = core.create_module("m2", rand_type(), &m2_cb);
        m2.borrow_mut().id = diff_id;

        core.send(&m1, &mut send_msg);

        wait_timeout!(called_rx, time::Duration::from_secs(1), || {
            assert!(false, "Callback was never called!")
        });
    }
    #[test]
    fn broadcast() {
        let mut send_msg = Message::broadcast(rand_command(), &rand_data(rand_data_size()));
        let gm1 = send_msg.clone();
        let gm2 = send_msg.clone();

        let (called_tx_1, called_rx_1) = Event::new();
        let (called_tx_2, called_rx_2) = Event::new();

        let m1_cb = move |msg: &Message| {
            assert_eq!(msg.header.command, gm1.header.command);
            assert_eq!(msg.data, gm1.data);
            called_tx_1.set();
        };
        let m2_cb = move |msg: &Message| {
            assert_eq!(msg.header.command, gm2.header.command);
            assert_eq!(msg.data, gm2.data);
            called_tx_2.set();
        };

        let mut core = Core::new();

        let m1 = core.create_module("m1", rand_type(), &m1_cb);
        m1.borrow_mut().id = rand_id();

        let m2 = core.create_module("m2", rand_type(), &m2_cb);
        m2.borrow_mut().id = rand_id();

        core.send(&m1, &mut send_msg);

        wait_timeout!(called_rx_1, time::Duration::from_secs(1), || {
            assert!(false, "Callback was never called!")
        });
        wait_timeout!(called_rx_2, time::Duration::from_secs(1), || {
            assert!(false, "Callback was never called!")
        });
    }
    fn rand_id_msg() -> Message {
        Message::id(rand_id(), rand_command(), &rand_data(rand_data_size()))
    }
    struct Event {
        flag: Rc<RefCell<bool>>,
    }
    impl Event {
        pub fn new() -> (Event, Event) {
            let flag_ref = Rc::new(RefCell::new(false));

            let tx = Event { flag: flag_ref.clone() };
            let rx = Event { flag: flag_ref.clone() };

            (tx, rx)
        }
        pub fn set(&self) {
            let mut flag = self.flag.borrow_mut();
            *flag = true;
        }
        pub fn is_set(&self) -> bool {
            *self.flag.borrow()
        }
    }
}
