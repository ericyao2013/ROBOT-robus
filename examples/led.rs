#![no_std]

#[cfg(not(target_arch = "arm"))]
extern crate std;

#[cfg(target_arch = "arm")]
const HEAP_SIZE: usize = 5000;

extern crate robus;

use robus::{Command, Message, ModuleType};

#[cfg(not(target_arch = "arm"))]
extern crate mockup_hal as hal;
#[cfg(target_arch = "arm")]
extern crate stm32f0_hal as hal;

use hal::gpio;

const LED_MODULE_ID: u16 = 3;
const PIN: gpio::Pin = gpio::Pin::PC7;
const BAUDRATE: u32 = 57600;

fn main() {
    #[cfg(target_arch = "arm")]
    hal::allocator::setup(HEAP_SIZE);

    let pin = gpio::Output::setup(PIN);
    let pin = core::cell::RefCell::new(pin);

    let cb = move |msg: Message| {
        match msg.header.command {
            Command::PublishState => {
                if msg.data[0] == 1 {
                    pin.borrow_mut().high();
                } else {
                    pin.borrow_mut().low();
                }
            }
            _ => (),
        };
    };

    let mut core = robus::init(BAUDRATE);

    let led = core.create_module("disco_led", ModuleType::Ledstrip, &cb);
    core.set_module_id(led, LED_MODULE_ID);

    loop {}
}
