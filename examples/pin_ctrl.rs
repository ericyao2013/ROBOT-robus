#![no_std]
#![feature(alloc)]

const ALIAS: &'static str = "mod_2";
const ID: u16 = 2;
const TYPE: ModuleType = ModuleType::GenericIO;
const ROBUS_BAUDRATE: u32 = 57_600;

extern crate robus;
use robus::{Command, Message, ModuleType};

#[cfg(not(target_arch = "arm"))]
extern crate mockup_hal as hal;
#[cfg(target_arch = "arm")]
extern crate stm32f0_hal as hal;

#[cfg(target_arch = "arm")]
const HEAP_SIZE: usize = 5000;

#[macro_use(vec)]
extern crate alloc;
use alloc::{String, Vec};

use hal::{adc, gpio};

fn ser_intro(alias: &str, mod_type: ModuleType) -> Vec<u8> {
    let mut v = String::from(alias).into_bytes();
    v.push(mod_type as u8);
    v
}
struct State {
    pin1: adc::Analog,
    pin2: gpio::Output,
    pin3: gpio::Output,
    pin4: gpio::Output,
    pin8: gpio::Input,
    pin9: gpio::Input,
    pin10: gpio::Input,
    pin11: gpio::Input,
    pin12: adc::Analog,
}
impl State {
    pub fn serialize(&self) -> Vec<u8> {
        vec![
            (self.pin1.read() >> 8) as u8,
            self.pin1.read() as u8,
            self.pin8.read() as u8,
            self.pin9.read() as u8,
            self.pin10.read() as u8,
            self.pin11.read() as u8,
            (self.pin12.read() >> 8) as u8,
            self.pin12.read() as u8,
        ]
    }
}

fn main() {
    #[cfg(target_arch = "arm")]
    hal::allocator::setup(HEAP_SIZE);

    // robus setup
    let (tx, rx) = robus::message_queue();
    let mut core = robus::init(ROBUS_BAUDRATE);

    // Analog pins setup
    let pin1 = adc::Analog::setup(adc::Pin::PA0);
    let pin12 = adc::Analog::setup(adc::Pin::PA1);

    // Output pins setup
    let pin2 = gpio::Output::setup(gpio::Pin::PB5);
    let pin3 = gpio::Output::setup(gpio::Pin::PB4);
    let pin4 = gpio::Output::setup(gpio::Pin::PB3);

    // Input pin setup
    let pin8 = gpio::Input::setup(gpio::Pin::PB11);
    let pin9 = gpio::Input::setup(gpio::Pin::PB10);
    let pin10 = gpio::Input::setup(gpio::Pin::PB1);
    let pin11 = gpio::Input::setup(gpio::Pin::PB0);

    //create struct
    let mut pins = State {
        pin1,
        pin2,
        pin3,
        pin4,
        pin8,
        pin9,
        pin10,
        pin11,
        pin12,
    };
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
                        pins.pin2.high();
                    }
                    if msg.data[0] == 0 {
                        pins.pin2.low();
                    }
                    // p3 value
                    if msg.data[1] == 1 {
                        pins.pin3.high();
                    }
                    if msg.data[1] == 0 {
                        pins.pin3.low();
                    }
                    // p4 value
                    if msg.data[2] == 1 {
                        pins.pin4.high();
                    }
                    if msg.data[2] == 0 {
                        pins.pin4.low();
                    }
                }
                _ => {}
            }
        }
    }
}
