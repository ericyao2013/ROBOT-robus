//! # Robus: the robust robotic bus
//!
//! Robus lets you seamlessly connect robotic actuators and sensors using a unified bus. New modules are automatically detected thanks to the topology detection algorithm.
//!
//! Each robus node is called a module and represent a function in your robot - typically a sensor or actuator. Modules can directly communicate with other modules using broadcast, specific module ids or specific module groups.
//!
//! Robus is light-weighted and designed for the embedded world.
//!
//! Robus reduces the time between an idea to the prototype. It provides a unified messaging architecture for modular robotics, all modules can be connected on the same 5 wires bus containing both power and 2 communication bus.

#![no_std]
#![feature(alloc)]
#![feature(never_type)]

extern crate embedded_hal as hal;

#[macro_use(as_static)]
extern crate as_static;
#[macro_use(block)]
extern crate nb;

#[macro_use(format)]
extern crate alloc;

#[cfg(not(target_arch = "arm"))]
extern crate std;

mod command;
mod collections;
mod error;
mod module;
mod msg;
mod recv_buf;
mod robus_core;

pub use command::Command;
pub use collections::message_queue;
pub use module::{Module, ModuleType};
pub use msg::Message;
pub use robus_core::Core;

use hal::digital::OutputPin;
use hal::serial;
use hal::timer::Timeout;

pub trait Peripherals {
    // Serial baudrate
    fn baudrate(&self) -> u32;

    // Serial message data
    fn rx(&mut self) -> &mut serial::AsyncRead<u8>;
    fn tx(&mut self) -> &mut serial::Write<u8, Error = !>;

    // Serial lock
    fn de(&mut self) -> &mut OutputPin;
    fn re(&mut self) -> &mut OutputPin;

    // Serial lock timeout
    fn timeout(&mut self) -> &mut Timeout<Time = u32>;
}

/// Init function to setup robus communication
///
/// Must be called before actually trying to read or send any `Message`.
pub fn init<P>(p: P) -> Core<P>
where
    P: Peripherals + 'static,
{
    Core::new(p)
}

// TODO: We still need to find a way not to use
// hardcoded USARTX for the interruption.
#[no_mangle]
pub extern "C" fn USART1() {
    unsafe {
        robus_core::serial_reception();
    }
}
#[no_mangle]
// TODO: We still need to find a way not to use
// hardcoded TIMX for the interruption.
pub extern "C" fn TIM7() {
    unsafe {
        robus_core::timeout();
    }
}
