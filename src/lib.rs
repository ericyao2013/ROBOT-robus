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

extern crate alloc;

#[cfg(target_arch = "arm")]
extern crate cortex_m;

#[cfg(target_arch = "arm")]
extern crate stm32f0_hal as hal;
#[cfg(target_arch = "arm")]
#[macro_use(interrupt)]
extern crate stm32f0x2 as ll;

#[cfg(not(target_arch = "arm"))]
#[macro_use(print)]
extern crate std;

mod command;
mod collections;
mod lock;
mod log;
mod module;
mod msg;
pub mod physical;
mod recv_buf;
mod robus_core;

pub use command::Command;
pub use collections::message_queue;
pub use log::LOGGER;
pub use module::{Module, ModuleType};
pub use msg::Message;
pub use robus_core::Core;


/// Init function to setup robus communication
///
/// Must be called before actually trying to read or send any `Message`.
pub fn init() -> Core {
    let mut core = Core::new();

    physical::setup(57600, |byte| core.receive(byte));
    physical::enable_interrupt();
    physical::setup_debug(57600);

    core
}
