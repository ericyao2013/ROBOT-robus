#![no_std]

const HEAP_SIZE: usize = 5000;

extern crate robus;
use robus::{Command, Message, ModuleType};

extern crate stm32f0_hal as hal;
use hal::prelude::*;

extern crate embedded_hal;
use embedded_hal::prelude::*;

const LED_MODULE_ID: u16 = 3;

struct P {}
impl robus::Peripherals for P {}

fn main() {
    hal::allocator::setup(HEAP_SIZE);

    let p = hal::stm32f0x2::Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();
    let mut gpioc = p.GPIOC.split(&mut rcc.ahb);
    let pin = gpioc
        .pc7
        .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);
    let pin = core::cell::RefCell::new(pin);

    let cb = move |msg: Message| {
        match msg.header.command {
            Command::PublishState => {
                if msg.data[0] == 1 {
                    pin.borrow_mut().set_high();
                } else {
                    pin.borrow_mut().set_low();
                }
            }
            _ => (),
        };
    };

    let mut core = robus::init(P {});

    let led = core.create_module("disco_led", ModuleType::Ledstrip, &cb);
    core.set_module_id(led, LED_MODULE_ID);

    loop {}
}
