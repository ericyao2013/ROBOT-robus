#![no_std]
#![feature(used)]
#![feature(lang_items)]
#![feature(global_allocator)]

#[cfg(not(target_arch = "arm"))]
#[macro_use(println, print)]
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
extern crate cortex_m_semihosting;
#[cfg(target_arch = "arm")]
extern crate cortex_m_rt;
#[cfg(target_arch = "arm")]
extern crate cortex_m;

#[cfg(target_arch = "arm")]
use cortex_m_semihosting::hio;
#[cfg(target_arch = "arm")]
use core::fmt::Write;

extern crate robus;

use robus::{Message, ModuleType};

fn main() {
    #[cfg(target_arch = "arm")]
    let start = unsafe { &mut _sheap as *mut u32 as usize };
    #[cfg(target_arch = "arm")] unsafe { ALLOCATOR.init(start, STACK_SIZE) }

    #[cfg(target_arch = "arm")]
    let mut stdout = hio::hstdout().unwrap();

    let (_, rx) = robus::message_queue();

    let mut cb = move |msg: &Message| {
        #[cfg(target_arch = "arm")] writeln!(stdout, "Receive {:?}", msg).unwrap();
        #[cfg(not(target_arch = "arm"))]
        println!("Receive {:?}", msg);
    };
    let mut core = robus::init();

    let button = core.create_module("bob", ModuleType::Button, &mut cb);

    loop {
        if let Some(mut msg) = rx.recv() {
            core.send(&button, &mut msg);
        }
    }
}
