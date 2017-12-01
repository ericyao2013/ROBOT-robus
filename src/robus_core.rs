use {Message, Module, ModuleType};

use msg::{MAX_MESSAGE_SIZE, TargetMode};
use recv_buf::RecvBuf;
use physical;

use core;
use alloc::vec::Vec;

static mut REGISTRY: Option<Vec<Module>> = None;

pub struct Core {
    recv_buf: RecvBuf,
}

impl Core {
    pub fn new() -> Core {
        unsafe {
            REGISTRY = Some(Vec::new());
        }

        Core { recv_buf: RecvBuf::with_capacity(MAX_MESSAGE_SIZE) }
    }
    pub fn create_module<'a>(
        &mut self,
        alias: &'a str,
        mod_type: ModuleType,
        cb: &'a Fn(Message),
    ) -> usize {
        let module = Module::new(alias, mod_type, cb);

        let reg = unsafe { get_registry() };
        unsafe {
            reg.push(extend_lifetime(module));
        }
        reg.len() - 1
    }
    // TODO: this function should probably be private only.
    pub fn set_module_id(&mut self, mod_id: usize, robus_id: u16) {
        let reg = unsafe { get_registry() };
        let module = &mut reg[mod_id];
        module.id = robus_id;
    }
    pub fn receive(&mut self, byte: u8) {
        self.recv_buf.push(byte);

        if let Some(msg) = self.recv_buf.get_message() {
            let reg = unsafe { get_registry() };

            let matches = match msg.header.target_mode {
                TargetMode::Broadcast => reg.iter().filter(|_| true).collect(),
                TargetMode::Id => {
                    reg.iter()
                        .filter(|module| module.id == msg.header.target)
                        .collect()
                }
                _ => Vec::new(),
            };

            for ref module in matches.iter() {
                (module.callback)(msg.clone());
            }
        }
    }
    pub fn send(&mut self, mod_id: usize, msg: &mut Message) {
        let reg = unsafe { get_registry() };
        let module = &reg[mod_id];
        msg.header.source = module.id;

        for byte in msg.to_bytes() {
            physical::send_when_ready(byte);

            // TODO: is this local loop a good idea?
            self.receive(byte);
        }
    }
}

unsafe fn get_registry() -> &'static mut Vec<Module<'static>> {
    if let Some(ref mut reg) = REGISTRY {
        reg
    } else {
        panic!("Core Module Registry not initialized!")
    }
}

unsafe fn extend_lifetime<'a>(f: Module<'a>) -> Module<'static> {
    core::mem::transmute::<Module<'a>, Module<'static>>(f)
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;

    use self::std::time;
    use self::std::rc::Rc;
    use self::std::cell::RefCell;

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
    fn fill_source_on_send() {
        let mut core = Core::new();
        let mut msg = rand_id_msg();

        let from = rand_id();

        let m1 = core.create_module("m1", rand_type(), &|_| {});
        core.set_module_id(m1, from);

        core.send(m1, &mut msg);

        assert_eq!(msg.header.source, from);
    }

    #[test]
    fn id_local_loop() {
        let mut send_msg = rand_id_msg();
        let gold_msg = send_msg.clone();

        let (called_tx, called_rx) = Event::new();

        let m1_cb = move |msg: Message| {
            assert_eq!(msg.header.command, gold_msg.header.command);
            assert_eq!(msg.data, gold_msg.data);
            called_tx.set();
        };
        let m2_cb = move |_msg: Message| {
            assert!(false);
        };

        let mut core = Core::new();

        let m1 = core.create_module("m1", rand_type(), &m1_cb);
        core.set_module_id(m1, send_msg.header.target);

        let mut diff_id = rand_id();
        while diff_id == send_msg.header.target {
            diff_id = rand_id();
        }
        let m2 = core.create_module("m2", rand_type(), &m2_cb);
        core.set_module_id(m2, diff_id);

        core.send(m1, &mut send_msg);

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

        let m1_cb = move |msg: Message| {
            assert_eq!(msg.header.command, gm1.header.command);
            assert_eq!(msg.data, gm1.data);
            called_tx_1.set();
        };
        let m2_cb = move |msg: Message| {
            assert_eq!(msg.header.command, gm2.header.command);
            assert_eq!(msg.data, gm2.data);
            called_tx_2.set();
        };

        let mut core = Core::new();

        let m1 = core.create_module("m1", rand_type(), &m1_cb);
        core.set_module_id(m1, rand_id());

        let m2 = core.create_module("m2", rand_type(), &m2_cb);
        core.set_module_id(m2, rand_id());

        core.send(m1, &mut send_msg);

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
