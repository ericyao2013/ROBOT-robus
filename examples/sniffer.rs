#![no_std]
#![feature(global_allocator)]

#[cfg(not(target_arch = "arm"))]
extern crate std;

#[cfg(target_arch = "arm")]
extern crate alloc_cortex_m0;
#[cfg(target_arch = "arm")]
use alloc_cortex_m0::CortexM0Heap;
#[cfg(target_arch = "arm")]
#[global_allocator]
static ALLOCATOR: CortexM0Heap = CortexM0Heap::empty();
#[cfg(target_arch = "arm")]
const STACK_SIZE: usize = 5000;

// These symbols come from a linker script
extern "C" {
    static mut _sheap: u32;
}
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
    let heap_start = unsafe { &mut _sheap as *mut u32 as usize };
    #[cfg(target_arch = "arm")]
    unsafe { ALLOCATOR.init(heap_start, STACK_SIZE) }

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
