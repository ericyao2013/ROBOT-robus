//! # Robus: the robust robotic bus
//!
//! Robus lets you seamlessly connect robotic actuators and sensors using a unified bus. New modules are automatically detected thanks to the topology detection algorithm.
//!
//! Each robus node is called a module and represent a function in your robot - typically a sensor or actuator. Modules can directly communicate with other modules using broadcast, specific module ids or specific module groups.
//!
//! Robus is lightweighted and designed for the embedded world.
//!
//! Robus reduces the time between an idea to the prototype. It provides a unified messaging achitecture for modular robotics, all modules can be connected on the same 5 wires bus containing both power and 2 communication bus.

extern crate mockup_hal as hal;
use hal::uart;

use std::cell::RefCell;
use std::rc::Rc;

mod command;
pub use command::Command;

mod module;
pub use module::{Module, ModuleType};

mod msg;
pub use msg::Message;

mod collections;
pub use collections::message_queue;

mod recv_buf;
use recv_buf::RecvBuf;

const RECV_BUF_SIZE: usize = msg::MAX_MESSAGE_SIZE;
static mut RECV_BUF_REF: Option<Rc<RefCell<RecvBuf>>> = None;

macro_rules! get_recv_buf {
    () => {
        unsafe {
            match RECV_BUF_REF {
                Some(ref mut r) => {
                    r.borrow_mut()
                },
                None => panic!("Could not access reception buffer!"),
            }
        }
    }
}

fn reception_cb(byte: u8) {
    let mut recv_buf = get_recv_buf!();

    recv_buf.push(byte);

    if let Some(msg) = recv_buf.get_message() {
        // TODO:
        //  - extract Id (and check target mode)
        //  - find module <==> Id
        //  - fire module.cb(&msg)
    }
}

/// Init function to setup robus communication
///
/// Must be called before actually trying to read or send any `Message`.
pub fn init() {
    unsafe {
        if let Some(ref _buf) = RECV_BUF_REF {
            panic!("You should only called init once!");
        }
    }

    let buf = RecvBuf::with_capacity(RECV_BUF_SIZE);
    let buf_ref = Rc::new(RefCell::new(buf));
    unsafe {
        RECV_BUF_REF = Some(buf_ref);
    }

    uart::setup(
        57600,
        uart::NBits::_8bits,
        uart::StopBits::_1b,
        uart::Parity::None,
        reception_cb,
    );
}
