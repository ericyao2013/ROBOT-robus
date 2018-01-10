#![no_std]
#![feature(alloc)]
const ALIAS: &'static str = "mod-2";
const ID: u16 = 2;
const TYPE: ModuleType = ModuleType::InputGPIO;
extern crate robus;
use robus::{Command, Message, ModuleType};
#[cfg(not(target_arch = "arm"))]
extern crate mockup_hal as hal;
#[cfg(target_arch = "arm")]
extern crate stm32f0_hal as hal;
#[cfg(target_arch = "arm")]
const HEAP_SIZE: usize = 5000;
extern crate alloc;
use alloc::{String, Vec};

use hal::gpio;
use hal::gpio::{Input, Output};

fn ser_intro(alias: &str, mod_type: ModuleType) -> Vec<u8> {
    let mut v = String::from(alias).into_bytes();
    v.push(mod_type as u8);
    v
}
struct State {
    pin1: Input,
    pin12: Output,
}
impl State {
    pub fn serialize(&self) -> Vec<u8> {
        let mut data_vec: Vec<u8> = Vec::new();
        data_vec.push(self.pin1.read() as u8);
        data_vec
    }
}
fn main() {
    #[cfg(target_arch = "arm")]
    hal::allocator::setup(HEAP_SIZE);

    // robus setup
    let (tx, rx) = robus::message_queue();
    let mut core = robus::init(57600);

    // Input pin setup
    let pin1 = gpio::Input::setup(gpio::Pin::PA0);
    let pin12 = gpio::Output::setup(gpio::Pin::PA1);
    let mut pins = State{pin1, pin12};
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
                },
                Command::GetState => {
                    let mut answer = Message::id(msg.header.source, Command::PublishState, &pins.serialize());
                    core.send(m, &mut answer);
                }
                Command::SetState => {
                    if msg.data[0] > 0{
                        pins.pin12.high();
                    } else {
                        pins.pin12.low();
                    }
                }
                _ => {}
            }
        }
    }
}
