#![no_std]
#![feature(alloc)]
#![feature(never_type)]

extern crate alloc;

extern crate embedded_hal;
extern crate nb;
extern crate robus;
extern crate stm32f0_hal;

extern crate dynamixel;
use dynamixel::motors::XL_320;

const ID1: u8 = 11;
const ID2: u8 = 12;

fn main() {
    stm32f0_hal::allocator::setup(5000);
    let p = setup(stm32f0_hal::stm32f0x2::Peripherals::take().unwrap());

    let serial = HalfDuplex::new(p.serial_tx, p.serial_rx, p.half_duplex_control_pin);
    let (tx, rx) = serial.split();

    let mut c = dynamixel::with_protocol_v2(rx, tx);
    loop {
        let pos = c.read_data(ID1, XL_320::PresentPosition).unwrap();
        c.write_data(ID2, XL_320::GoalPosition, pos).unwrap();
    }
}

use stm32f0_hal::serial::{Rx, Tx};
use stm32f0_hal::stm32f0x2::USART3;
use stm32f0_hal::prelude::*;
use stm32f0_hal::gpio::{Output, PushPull};
use stm32f0_hal::gpio::gpiob::PB1;
use embedded_hal::digital::OutputPin;

struct Peripherals {
    serial_tx: Tx<USART3>,
    serial_rx: Rx<USART3>,
    half_duplex_control_pin: PB1<Output<PushPull>>,
}
fn setup(p: stm32f0_hal::stm32f0x2::Peripherals) -> Peripherals {
    let mut flash = p.FLASH.constrain();

    let mut rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = p.GPIOB.split(&mut rcc.ahb);

    let pb1 = gpiob
        .pb1
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let pb10 = gpiob.pb10.into_alternate_push_pull(
        &mut gpiob.moder,
        &mut gpiob.afr,
        stm32f0_hal::gpio::AF4,
    );
    let pb11 = gpiob.pb11.into_alternate_push_pull(
        &mut gpiob.moder,
        &mut gpiob.afr,
        stm32f0_hal::gpio::AF4,
    );

    let serial = stm32f0_hal::serial::Serial::usart3(
        p.USART3,
        (pb10, pb11),
        57_600_u32.bps(),
        clocks,
        &mut rcc.apb1,
    );
    let (serial_tx, serial_rx) = serial.split();

    Peripherals {
        serial_tx,
        serial_rx,
        half_duplex_control_pin: pb1,
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Read,
    Write,
}
struct HalfDuplex {
    tx: Tx<USART3>,
    rx: Rx<USART3>,
    control_pin: PB1<Output<PushPull>>,
    mode: Mode,
}
impl HalfDuplex {
    fn new(tx: Tx<USART3>, rx: Rx<USART3>, control_pin: PB1<Output<PushPull>>) -> HalfDuplex {
        let mode = Mode::Read;

        let mut hd = HalfDuplex {
            tx,
            rx,
            control_pin,
            mode,
        };
        hd.switch(mode);
        hd
    }
    fn split(self) -> (Wrapper, Wrapper) {
        let cell = Rc::new(RefCell::new(self));
        let tx = Wrapper(cell.clone());
        let rx = Wrapper(cell.clone());
        (tx, rx)
    }
    fn switch(&mut self, mode: Mode) {
        if self.mode != mode {
            match mode {
                // TODO: en random
                Mode::Read => self.control_pin.set_low(),
                Mode::Write => self.control_pin.set_high(),
            }
            self.mode = mode;
        }
    }
}

use core::cell::RefCell;
use core::ops::Deref;
use alloc::rc::Rc;

struct Wrapper(Rc<RefCell<HalfDuplex>>);
impl Deref for Wrapper {
    type Target = RefCell<HalfDuplex>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl embedded_hal::serial::Read<u8> for Wrapper {
    type Error = !;
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.borrow_mut().switch(Mode::Read);
        self.borrow_mut().rx.read()
    }
}
impl embedded_hal::serial::Write<u8> for Wrapper {
    type Error = !;
    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        self.borrow_mut().switch(Mode::Write);
        self.borrow_mut().tx.write(byte)
    }
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.borrow_mut().tx.flush()
    }
    fn complete(&self) -> nb::Result<(), Self::Error> {
        self.borrow_mut().tx.complete()
    }
}
