#![no_std]
#![feature(alloc)]

#[cfg(not(target_arch = "arm"))]
extern crate std;

#[cfg(not(target_arch = "arm"))]
extern crate mockup_hal as hal;
#[cfg(target_arch = "arm")]
extern crate stm32f0_hal as hal;

#[cfg(target_arch = "arm")]
const HEAP_SIZE: usize = 5000;

extern crate alloc;
use alloc::vec::Vec;

use core::fmt::Write;

#[macro_use]
extern crate robus;

use robus::{Command, Message, ModuleType};

const ID: u16 = 1;
const NB_MODULES: u16 = 4;
const NEXT: u16 = ID % NB_MODULES + 1;
const BAUDRATE: u32 = 57600;

fn main() {
    #[cfg(target_arch = "arm")]
    hal::allocator::setup(HEAP_SIZE);

    let (tx, rx) = robus::message_queue();
    let cb = |msg: Message| match msg.header.command {
        Command::Introduction => tx.send(msg),
        _ => {}
    };

    let mut core = robus::init(BAUDRATE);
    let module = core.create_module("mod", ModuleType::Button, &cb);
    core.set_module_id(module, ID);

    let mut send_msg = Message::id(NEXT, Command::Introduction, &Vec::new());

    // If we are the first on the chain:
    // We wait to make sure everyone is ready,
    // And then we send the first message to initiate the loop.
    if ID == 1 {
        hal::rcc::ms_delay(1000);
        core.send(module, &mut send_msg);
        log!("Start!");
    }

    loop {
        if let Some(_) = rx.recv() {
            core.send(module, &mut send_msg);

            if ID == 1 {
                log!("Loop!");
            }
        }
    }
}
