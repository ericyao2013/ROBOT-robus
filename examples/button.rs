#![no_std]
#![feature(alloc)]

static HEAP_SIZE: usize = 5000;

#[macro_use(vec)]
extern crate alloc;

extern crate robus;

use robus::{Command, Message, ModuleType};

extern crate stm32f0_hal as hal;
use hal::prelude::*;

extern crate embedded_hal;
use embedded_hal::prelude::*;

extern crate cortex_m;

const BUTTON_MODULE_ID: u16 = 2;
const LED_MODULE_ID: u16 = 3;

struct P {}
impl robus::Peripherals for P {}

fn main() {
    hal::allocator::setup(HEAP_SIZE);

    let p = hal::stm32f0x2::Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
    let pin = gpioa
        .pa0
        .into_floating_input(&mut gpioa.moder, &mut gpioa.pupdr);

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    let mut core = robus::init(P {});
    let button = core.create_module("fire_button", ModuleType::Button, &|_| {});
    core.set_module_id(button, BUTTON_MODULE_ID);

    let mut msg = Message::id(LED_MODULE_ID, Command::PublishState, &vec![0]);
    loop {
        msg.data[0] = pin.is_high() as u8;
        core.send(button, &mut msg);

        delay.delay_ms(100_u16);
    }
}
