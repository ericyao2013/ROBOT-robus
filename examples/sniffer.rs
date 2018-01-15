#![no_std]

#[cfg(not(target_arch = "arm"))]
extern crate std;

#[cfg(target_arch = "arm")]
extern crate stm32f0_hal as hal;
#[cfg(target_arch = "arm")]
const HEAP_SIZE: usize = 5000;

#[cfg(target_arch = "arm")]
extern crate cortex_m;
#[cfg(target_arch = "arm")]
extern crate cortex_m_rt;

use core::fmt::Write;

#[macro_use]
extern crate robus;

use robus::{Message, ModuleType};

fn main() {
    #[cfg(target_arch = "arm")]
    hal::allocator::setup(HEAP_SIZE);

    let (tx, rx) = robus::message_queue();
    let cb = move |msg: Message| {
        tx.send(msg);
    };

    let mut core = robus::init();
    core.create_module("logger", ModuleType::Sniffer, &cb);

    loop {
        if let Some(msg) = rx.recv() {
            log!("Got {:?}", msg);
        }
    }
}
