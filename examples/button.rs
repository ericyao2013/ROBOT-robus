#![no_std]
#![feature(alloc)]

#[cfg(not(target_arch = "arm"))]
extern crate std;

#[cfg(target_arch = "arm")]
static HEAP_SIZE: usize = 5000;

#[macro_use(vec)]
extern crate alloc;
use alloc::vec::Vec;

extern crate robus;

use robus::{Command, Message, ModuleType};

#[cfg(not(target_arch = "arm"))]
extern crate mockup_hal as hal;
#[cfg(target_arch = "arm")]
extern crate stm32f0_hal as hal;

use hal::{gpio, rcc};

const BUTTON_MODULE_ID: u16 = 2;
const LED_MODULE_ID: u16 = 3;
const PIN: gpio::Pin = gpio::Pin::PA0;
const BAUDRATE: u32 = 57600;

fn main() {
    #[cfg(target_arch = "arm")]
    hal::allocator::setup(HEAP_SIZE);

    let mut core = robus::init(BAUDRATE);

    let button = core.create_module("fire_button", ModuleType::Button, &|_| {});
    core.set_module_id(button, BUTTON_MODULE_ID);
    let pin = gpio::Input::setup(PIN);

    let mut msg = Message::id(LED_MODULE_ID, Command::PublishState, &vec![0]);
    loop {
        msg.data[0] = pin.read() as u8;
        core.send(button, &mut msg);

        rcc::ms_delay(100);
    }
}
