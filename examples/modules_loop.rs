#![no_std]
#![feature(alloc)]
#![feature(never_type)]

extern crate alloc;
use alloc::vec::Vec;
const HEAP_SIZE: usize = 5000;

extern crate robus;
use robus::{Command, Message, ModuleType};

extern crate stm32f0_hal as hal;
use hal::gpio::{Output, PushPull};
use hal::gpio::gpiob::{PB14, PB15};
use hal::serial::Serial;
use hal::prelude::*;

extern crate embedded_hal;
use embedded_hal::digital::OutputPin;
use embedded_hal::serial;
use embedded_hal::prelude::*;

extern crate cortex_m;

struct RobusPeripherals<TX>
where
    TX: serial::Write<u8, Error = !>,
{
    tx: TX,
    de: PB14<Output<PushPull>>,
    re: PB15<Output<PushPull>>,
}

impl<TX> robus::Peripherals for RobusPeripherals<TX>
where
    TX: serial::Write<u8, Error = !>,
{
    fn tx(&mut self) -> &mut serial::Write<u8, Error = !> {
        &mut self.tx
    }
    fn de(&mut self) -> &mut OutputPin {
        &mut self.de
    }
    fn re(&mut self) -> &mut OutputPin {
        &mut self.re
    }
}

const ID: u16 = 1;
const NB_MODULES: u16 = 4;
const NEXT: u16 = ID % NB_MODULES + 1;

fn main() {
    hal::allocator::setup(HEAP_SIZE);

    let p = hal::stm32f0x2::Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb);

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    let (tx, rx) = robus::message_queue();
    let cb = |msg: Message| match msg.header.command {
        Command::Introduction => tx.send(msg),
        _ => {}
    };

    let de = gpiob
        .pb14
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let re = gpiob
        .pb15
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let pa9 = gpioa
        .pa9
        .into_alternate_push_pull(&mut gpioa.moder, &mut gpioa.afr, hal::gpio::AF1);
    let pa10 =
        gpioa
            .pa10
            .into_alternate_push_pull(&mut gpioa.moder, &mut gpioa.afr, hal::gpio::AF1);
    let serial = Serial::usart1(
        p.USART1,
        (pa9, pa10),
        57_600_u32.bps(),
        clocks,
        &mut rcc.apb2,
    );
    let (tx, _rx) = serial.split();

    let peripherals = RobusPeripherals { tx, de, re };

    let mut core = robus::init(peripherals);
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
