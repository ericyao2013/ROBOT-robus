//! Robus core - handles the intern mechanisms for creating modules and dispatch them the received messages.

use {Message, Module, ModuleType, Peripherals};

use msg::TargetMode;
use recv_buf;

use core;
use core::mem;
use core::ops::DerefMut;

use alloc::vec::Vec;

use hal::serial;
use hal::timer;

pub static mut TX_LOCK: bool = false;

static mut REGISTRY: Option<Vec<Module>> = None;

as_static!(RECV_CB: &mut FnMut(u8));
as_static!(RX: &mut serial::AsyncRead<u8>);

as_static!(TIMEOUT: &mut timer::Timeout<Time = u32>);
as_static!(TIMEOUT_DT: u32);
static TIMEOUT_FACTOR: u32 = 2;

/// Handles the intern mechanisms for creating modules and dispatch them the received messages.
///
/// The Core is reponsible for:
///
/// * handling the hardware communication with the bus
/// * creating new Module
/// * dispatching Message to the targeted Module
///
///
/// Note: *Only one Core should be created as it handles the hardware configuration (e.g. UART interruption).*
pub struct Core<P>
where
    P: Peripherals,
{
    p: P,
}

impl<P: 'static> Core<P>
where
    P: Peripherals,
{
    /// Creates a `Core` and setup the Module registry and the reception buffer.
    ///
    /// Note: *Only one Core should be created as it handles the hardware configuration (e.g. UART interruption).*
    /// TODO: We should make the Core a singleton or panic! if called multiple times.
    pub fn new<'a>(p: P) -> Core<P> {
        unsafe {
            REGISTRY = Some(Vec::new());
        }

        let mut core = Core { p };

        unsafe {
            core.p.rx().listen();
            RX.lazy_init(mem::transmute(core.p.rx()));
            listen_serial_interrupt(&mut |b| core.receive(b));

            core.p.timeout().listen(timer::Event::Fired);
            TIMEOUT.lazy_init(mem::transmute(core.p.timeout()));

            let baudrate = core.p.baudrate();
            let dt = baudrate/(10 * TIMEOUT_FACTOR); //each byte need 10 bits and we want a timeout of TIMEOUT_FACTOR x bytes
            TIMEOUT_DT.lazy_init(dt);
        }

        // Default RX Enable -> /RE = 0 & DE = 0
        core.p.de().set_low();
        core.p.re().set_low();

        core
    }
    /// Create a new `Module` attached with the Robus `Core`.
    ///
    /// # Arguments
    /// * `alias`: a `&str` representing the name of the `Module`
    /// * `mod_type`: the `ModuleType` caracterising the `Module`
    /// * `cb`: the reception callback `Fn(Message)` called each time a `Message` targetting this module is received.
    pub fn create_module<'a>(
        &mut self,
        alias: &'a str,
        mod_type: ModuleType,
        cb: &'a Fn(Message),
    ) -> usize {
        let module = Module::new(alias, mod_type, cb);

        let reg = unsafe { get_registry() };
        unsafe {
            reg.push(core::mem::transmute(module));
        }
        reg.len() - 1
    }
    /// Change the module id used on the bus
    ///
    /// # Arguments
    /// * `mod_id`: the internal id `usize` used by the `Core` to identify a `Module`
    /// * `robus_id`: a `u16` id identifying the `Module` on the bus. It is typically determined by the topology detection.
    ///
    /// Note: *The bus id is global to the whole bus and may thus differ from the local id used for the module registry.*
    ///
    /// TODO: this function should probably be private only (kept for testing purpose).
    pub fn set_module_id(&mut self, mod_id: usize, robus_id: u16) {
        let reg = unsafe { get_registry() };
        let module = &mut reg[mod_id];
        module.id = robus_id;
    }
    /// Robus byte reception callback
    ///
    /// # Arguments
    /// * `byte`: the received `u8` byte
    ///
    fn receive(&mut self, byte: u8) {
        unsafe {
            core::ptr::write_volatile(&mut TX_LOCK, true);
        }

        recv_buf::push(byte);

        if let Some(msg) = recv_buf::get_message() {
            let reg = unsafe { get_registry() };

            let matches = match msg.header.target_mode {
                TargetMode::Broadcast => reg.iter().filter(|_| true).collect(),
                TargetMode::Id => reg.iter()
                    .filter(|module| {
                        module.id == msg.header.target || module.mod_type == ModuleType::Sniffer
                    })
                    .collect(),
                _ => Vec::new(),
            };

            for ref module in matches.iter() {
                // TODO: could we use a ref instead?
                (module.callback)(msg.clone());
            }
        }
    }
    /// Send a `Message` on the bus
    ///
    /// # Arguments
    /// * `mod_id`: the `usize` id of the sending `Module`
    /// * `msg`: the `Message` to send (needs to be mut as we will inject the source inside)
    ///
    pub fn send(&mut self, mod_id: usize, msg: &mut Message) {
        let reg = unsafe { get_registry() };
        let module = &reg[mod_id];
        msg.header.source = module.id;

        // Wait TX to be free & lock
        // TODO: write a mutex abstraction
        unsafe {
            while core::ptr::read_volatile(&TX_LOCK) {}
            core::ptr::write_volatile(&mut TX_LOCK, true);
        }

        // TX Enable -> \RE = 1 & DE = 1
        self.p.re().set_high();
        self.p.de().set_high();

        for byte in msg.to_bytes() {
            block!(self.p.tx().write(byte)).ok();
        }

        // RX Enable -> \RE = 0 & DE = 0
        self.p.re().set_low();
        self.p.de().set_low();

        unsafe {
            TIMEOUT.start(*TIMEOUT_DT);
        }

        #[cfg(test)]
        // Use a local loop for unit-testing
        for byte in msg.to_bytes() {
            self.receive(byte);
        }
    }
}

pub unsafe fn listen_serial_interrupt<F>(mut f: F)
where
    F: FnMut(u8),
{
    RECV_CB.lazy_init(core::mem::transmute::<
        &mut FnMut(u8),
        &'static mut FnMut(u8),
    >(&mut f));
}

pub unsafe fn timeout() {
    core::ptr::write_volatile(&mut TX_LOCK, false);
    recv_buf::flush();
}

pub unsafe fn serial_reception() {
    let b = RX.async_read();

    TIMEOUT.start(*TIMEOUT_DT);
    RECV_CB.deref_mut()(b);
}

unsafe fn get_registry() -> &'static mut Vec<Module<'static>> {
    if let Some(ref mut reg) = REGISTRY {
        reg
    } else {
        panic!("Core Module Registry not initialized!")
    }
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

        wait_timeout!(called_rx, time::Duration::from_secs(1), || assert!(
            false,
            "Callback was never called!"
        ));
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

        wait_timeout!(called_rx_1, time::Duration::from_secs(1), || assert!(
            false,
            "Callback was never called!"
        ));
        wait_timeout!(called_rx_2, time::Duration::from_secs(1), || assert!(
            false,
            "Callback was never called!"
        ));
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

            let tx = Event {
                flag: flag_ref.clone(),
            };
            let rx = Event {
                flag: flag_ref.clone(),
            };

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
