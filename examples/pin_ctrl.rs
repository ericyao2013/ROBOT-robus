#![no_std]
#![feature(alloc)]
#![feature(never_type)]

const ALIAS: &'static str = "mod_2";
const ID: u16 = 2;
const TYPE: ModuleType = ModuleType::GenericIO;

extern crate robus;
use robus::{Command, Message, ModuleType};

#[macro_use(vec)]
extern crate alloc;
use alloc::{String, Vec};
const HEAP_SIZE: usize = 5000;

#[macro_use(block)]
extern crate nb;

extern crate stm32f0_hal as hal;
use hal::gpio::{Output, PushPull};
use hal::gpio::gpiob::{PB14, PB15};
use hal::serial::Serial;
use hal::timer::Timer;
use hal::prelude::*;
use hal::adc::Adc;

extern crate embedded_hal;
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal::serial;
use embedded_hal::timer;

struct RobusPeripherals<RX, TX, TIMEOUT>
where
    RX: serial::AsyncRead<u8>,
    TX: serial::Write<u8, Error = !>,
    TIMEOUT: timer::Timeout<Time = u32>,
{
    bps: u32,

    rx: RX,
    tx: TX,

    de: PB14<Output<PushPull>>,
    re: PB15<Output<PushPull>>,

    timeout: TIMEOUT,
}

impl<RX, TX, TIMEOUT> robus::Peripherals for RobusPeripherals<RX, TX, TIMEOUT>
where
    RX: serial::AsyncRead<u8>,
    TX: serial::Write<u8, Error = !>,
    TIMEOUT: timer::Timeout<Time = u32>,
{
    fn baudrate(&self) -> u32 {
        self.bps
    }
    fn rx(&mut self) -> &mut serial::AsyncRead<u8> {
        &mut self.rx
    }
    fn tx(&mut self) -> &mut serial::Write<u8, Error = !> {
        &mut self.tx
    }
    fn de(&mut self) -> &mut OutputPin {
        &mut self.de
    }
    fn re(&mut self) -> &mut OutputPin {
        &mut self.re
    }
    fn timeout(&mut self) -> &mut timer::Timeout<Time = u32> {
        &mut self.timeout
    }
}

fn ser_intro(alias: &str, mod_type: ModuleType) -> Vec<u8> {
    let mut v = String::from(alias).into_bytes();
    v.push(mod_type as u8);
    v
}

struct State<'a, P1, P2, P3, P4, P5, P6, P7, P8, P9>
where
    P1: Adc,
    P2: OutputPin,
    P3: OutputPin,
    P4: OutputPin,
    P5: InputPin,
    P6: InputPin,
    P7: InputPin,
    P8: InputPin,
    P9: Adc,
{
    pin1: P1,
    pin2: P2,
    pin3: P3,
    pin4: P4,
    pin5: P5,
    pin6: P6,
    pin7: P7,
    pin8: P8,
    pin9: P9,
    apb2: &'a mut hal::rcc::APB2,
}
impl<'a, P1, P2, P3, P4, P5, P6, P7, P8, P9> State<'a, P1, P2, P3, P4, P5, P6, P7, P8, P9>
where
    P1: Adc,
    P2: OutputPin,
    P3: OutputPin,
    P4: OutputPin,
    P5: InputPin,
    P6: InputPin,
    P7: InputPin,
    P8: InputPin,
    P9: Adc,
{
    pub fn serialize(&mut self) -> Vec<u8> {
        self.pin1.start(&mut self.apb2);
        let p1 = block!(self.pin1.read()).unwrap();

        self.pin9.start(&mut self.apb2);
        let p9 = block!(self.pin1.read()).unwrap();

        vec![
            (p1 >> 8) as u8,
            p1 as u8,
            self.pin5.is_high() as u8,
            self.pin6.is_high() as u8,
            self.pin7.is_high() as u8,
            self.pin8.is_high() as u8,
            (p9 >> 8) as u8,
            p9 as u8,
        ]
    }
}

fn main() {
    hal::allocator::setup(HEAP_SIZE);

    let p = hal::stm32f0x2::Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();
    let mut flash = p.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb);

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

    let bps = 57_600_u32.bps();
    let serial = Serial::usart1(p.USART1, (tx, rx), bps, clocks, &mut rcc.apb2);
    let (tx, rx) = serial.split();

    let timeout = Timer::tim7(p.TIM7, clocks, &mut rcc.apb1);

    let peripherals = RobusPeripherals {
        bps: bps.0,
        rx,
        tx,
        de,
        re,
        timeout,
    };

    let mut core = robus::init(peripherals);

    // Analog pins setup
    let pin1 = gpioa.pa0.into_analog(&mut gpioa.moder);
    let pin9 = gpioa.pa1.into_analog(&mut gpioa.moder);

    // Output pins setup
    let pin2 = gpiob
        .pb5
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let pin3 = gpiob
        .pb4
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let pin4 = gpiob
        .pb3
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    // Input pin setup
    let pin5 = gpiob
        .pb11
        .into_floating_input(&mut gpiob.moder, &mut gpiob.pupdr);
    let pin6 = gpiob
        .pb10
        .into_floating_input(&mut gpiob.moder, &mut gpiob.pupdr);
    let pin7 = gpiob
        .pb1
        .into_floating_input(&mut gpiob.moder, &mut gpiob.pupdr);
    let pin8 = gpiob
        .pb0
        .into_floating_input(&mut gpiob.moder, &mut gpiob.pupdr);

    let mut pins = State {
        pin1,
        pin2,
        pin3,
        pin4,
        pin5,
        pin6,
        pin7,
        pin8,
        pin9,
        apb2: &mut rcc.apb2,
    };
    let (tx, rx) = robus::message_queue();
    let m = core.create_module(ALIAS, TYPE, &|msg| {
        tx.send(msg);
    });
    core.set_module_id(m, ID);
    loop {
        if let Some(msg) = rx.recv() {
            match msg.header.command {
                Command::Identify => {
                    let data = ser_intro(ALIAS, TYPE);
                    let mut answer = Message::id(msg.header.source, Command::Introduction, &data);
                    core.send(m, &mut answer);
                }
                Command::GetState => {
                    let mut answer =
                        Message::id(msg.header.source, Command::PublishState, &pins.serialize());
                    core.send(m, &mut answer);
                }
                Command::SetState => {
                    // p2 value
                    if msg.data[0] == 1 {
                        pins.pin2.set_high();
                    }
                    if msg.data[0] == 0 {
                        pins.pin2.set_low();
                    }
                    // p3 value
                    if msg.data[1] == 1 {
                        pins.pin3.set_high();
                    }
                    if msg.data[1] == 0 {
                        pins.pin3.set_low();
                    }
                    // p4 value
                    if msg.data[2] == 1 {
                        pins.pin4.set_high();
                    }
                    if msg.data[2] == 0 {
                        pins.pin4.set_low();
                    }
                }
                _ => {}
            }
        }
    }
}
