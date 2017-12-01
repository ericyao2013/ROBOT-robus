#![no_std]
#![feature(alloc)]
#![feature(global_allocator)]

#[cfg(not(target_arch = "arm"))]
extern crate std;

extern crate alloc;
use alloc::vec::Vec;

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

use robus::{Command, Message, ModuleType};

#[cfg(target_arch = "arm")]
extern crate stm32f0_hal as hal;
#[cfg(not(target_arch = "arm"))]
extern crate mockup_hal as hal;

use hal::{gpio, rcc};

const BUTTON_MODULE_ID: u16 = 2;
const LED_MODULE_ID: u16 = 3;
const PIN: gpio::Pin = gpio::Pin::PA0;

fn main() {
    #[cfg(target_arch = "arm")]
    let heap_start = unsafe { &mut _sheap as *mut u32 as usize };
    #[cfg(target_arch = "arm")] unsafe { ALLOCATOR.init(heap_start, STACK_SIZE) }

    let mut core = robus::init();

    let button = core.create_module("fire_button", ModuleType::Button, &|_| {});
    core.set_module_id(button, BUTTON_MODULE_ID);
    let pin = gpio::Input::setup(PIN);

    let mut msg = Message::id(LED_MODULE_ID, Command::PublishState, &Vec::with_capacity(1));
    loop {
        msg.data[0] = pin.read() as u8;
        core.send(button, &mut msg);

        rcc::ms_delay(100);
    }
}
