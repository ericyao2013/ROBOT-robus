#![no_std]
#![feature(alloc)]

extern crate alloc;
use alloc::vec::Vec;
const HEAP_SIZE: usize = 5000;

extern crate robus;
use robus::{Command, Message, ModuleType};

extern crate stm32f0_hal as hal;
use hal::prelude::*;

extern crate embedded_hal;
use embedded_hal::prelude::*;

extern crate cortex_m;

const ID: u16 = 1;
const NB_MODULES: u16 = 4;
const NEXT: u16 = ID % NB_MODULES + 1;

struct P {}
impl robus::Peripherals for P {}

fn main() {
    hal::allocator::setup(HEAP_SIZE);

    let p = hal::stm32f0x2::Peripherals::take().unwrap();
    let rcc = p.RCC.constrain();

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    let (tx, rx) = robus::message_queue();
    let cb = |msg: Message| match msg.header.command {
        Command::Introduction => tx.send(msg),
        _ => {}
    };

    let mut core = robus::init(P {});
    let module = core.create_module("mod", ModuleType::Button, &cb);
    core.set_module_id(module, ID);

    let mut send_msg = Message::id(NEXT, Command::Introduction, &Vec::new());

    // If we are the first on the chain:
    // We wait to make sure everyone is ready,
    // And then we send the first message to initiate the loop.
    if ID == 1 {
        delay.delay_ms(250_u16);
        core.send(module, &mut send_msg);
    }

    loop {
        if let Some(_) = rx.recv() {
            core.send(module, &mut send_msg);
        }
    }
}
