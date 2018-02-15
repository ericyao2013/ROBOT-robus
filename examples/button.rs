#![no_std]
#![feature(alloc)]
#![feature(never_type)]

#[macro_use(vec)]
extern crate alloc;
static HEAP_SIZE: usize = 5000;

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

const BUTTON_MODULE_ID: u16 = 2;
const LED_MODULE_ID: u16 = 3;

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

fn main() {
    hal::allocator::setup(HEAP_SIZE);

    let p = hal::stm32f0x2::Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb);

    let pin = gpioa
        .pa0
        .into_floating_input(&mut gpioa.moder, &mut gpioa.pupdr);

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    let de = gpiob
        .pb14
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let re = gpiob
        .pb15
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let tx = gpioa
        .pa9
        .into_alternate_push_pull(&mut gpioa.moder, &mut gpioa.afr, hal::gpio::AF1);
    let rx = gpioa
        .pa10
        .into_alternate_push_pull(&mut gpioa.moder, &mut gpioa.afr, hal::gpio::AF1);
    let serial = Serial::usart1(p.USART1, (tx, rx), 57_600_u32.bps(), clocks, &mut rcc.apb2);
    let (tx, _rx) = serial.split();

    let peripherals = RobusPeripherals { tx, de, re };

    let mut core = robus::init(peripherals);
    let button = core.create_module("fire_button", ModuleType::Button, &|_| {});
    core.set_module_id(button, BUTTON_MODULE_ID);

    let mut msg = Message::id(LED_MODULE_ID, Command::PublishState, &vec![0]);
    loop {
        msg.data[0] = pin.is_high() as u8;
        core.send(button, &mut msg);

        delay.delay_ms(100_u16);
    }
}
