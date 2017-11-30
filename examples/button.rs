#![no_std]
#![feature(used)]
#![feature(alloc)]
#![feature(lang_items)]
#![feature(global_allocator)]

#[macro_use(vec)]
extern crate alloc;

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

#[cfg(target_arch = "arm")]
extern crate stm32f0_hal as hal;
//#[cfg(not(target_arch = "arm"))]
//extern crate mockup_hal as hal;

use hal::{gpio, rcc};

const BUTTON_MODULE_ID: u16 = 2;
const LED_MODULE_ID: u16 = 3;
const PIN: gpio::Pin = gpio::Pin::PA0;

fn main() {
    #[cfg(target_arch = "arm")]
    let start = unsafe { &mut _sheap as *mut u32 as usize };
    #[cfg(target_arch = "arm")] unsafe { ALLOCATOR.init(start, STACK_SIZE) }

    let pin = gpio::Input::setup(PIN);

    let cb = move |_msg: Message| {};

    let mut core = robus::init();

    let button = core.create_module("fire_button", robus::ModuleType::Button, &cb);
    core.set_module_id(button, BUTTON_MODULE_ID);

    loop {
        let mut msg = Message::id(
            LED_MODULE_ID,
            Command::PublishState,
            &vec![pin.read() as u8],
        );
        core.send(button, &mut msg);
        rcc::ms_delay(100);
    }
}
