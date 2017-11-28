#![no_std]
#![feature(used)]
#![feature(alloc)]
#![feature(lang_items)]
#![feature(global_allocator)]

#[macro_use(vec)]
extern crate alloc;
use alloc::borrow::ToOwned;

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

extern crate robus;

use robus::{Command, Message};

const GATE_ID: u16 = 1;

fn main() {
    #[cfg(target_arch = "arm")]
    let start = unsafe { &mut _sheap as *mut u32 as usize };
    #[cfg(target_arch = "arm")] unsafe { ALLOCATOR.init(start, STACK_SIZE) }

    let (tx, rx) = robus::message_queue();

    let cb = move |msg: Message| {
        let answer = match msg.header.command {
            Command::Identify => Some(Message::id(
                GATE_ID,
                Command::Introduction,
                &"hello".as_bytes().to_owned(), // TODO: read some real data here
            )),
            Command::GetState => Some(Message::id(
                GATE_ID,
                Command::PublishState,
                &vec![0], // TODO: read some real data here
            )),
            _ => None,
        };
        if let Some(answer) = answer {
            tx.send(answer);
        }
    };

    let mut core = robus::init();

    let button = core.create_module("fire_button", robus::ModuleType::Button, &cb);

    loop {
        if let Some(mut msg) = rx.recv() {
            core.send(button, &mut msg);
        }
    }
}
