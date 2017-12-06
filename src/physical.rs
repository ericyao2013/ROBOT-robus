//! Handles the physical communication with the bus.
//!
//! This module handles the physical aspect of the communication with the bus. In particular, it correctly sets the UART communication and the associated GPIOs.


#[cfg(target_arch = "arm")]
mod hard {
    use core;

    use hal::rcc;
    use ll::{USART1 as UART1, USART3 as UART3, GPIOA, GPIOB, NVIC, RCC};
    use ll::interrupt::*;
    use cortex_m;

    const FREQUENCY: u32 = 48000000;

    static mut DATA_UART1: u16 = 0;

    /// Setup the physical communication with the bus
    ///
    /// # Arguments
    ///
    /// * `baudrate` - A u32 specifying the communication baudrate
    /// * `f` - A `FnMut(u8)` reception callback - *WARNING: it will be called inside the interruption!*
    pub fn setup<F>(baudrate: u32, mut f: F)
    where
        F: FnMut(u8),
    {
        rcc::init();
        cortex_m::interrupt::free(|cs| {
            let rcc = RCC.borrow(cs);
            let gpioa = GPIOA.borrow(cs);
            let gpiob = GPIOB.borrow(cs);
            let uart = UART1.borrow(cs);

            // Enable GPIOA & GPIOB Clock
            rcc.ahbenr.modify(|_, w| w.iopaen().enabled());
            rcc.ahbenr.modify(|_, w| w.iopben().enabled());
            // Enable USART1 Clock
            rcc.apb2enr.write(|w| w.usart1en().enabled());
            // Configure PTPA (PA8) et PTPB (PB13) as input with pull-up
            gpioa.moder.modify(|_, w| w.moder8().input());
            gpioa.pupdr.modify(|_, w| w.pupdr8().pull_up());
            gpiob.moder.modify(|_, w| w.moder13().input());
            gpiob.pupdr.modify(|_, w| w.pupdr13().pull_up());
            // Configure DE (PB15) /RE (PB14) pin as output
            gpiob.moder.modify(
                |_, w| w.moder14().output().moder15().output(),
            );
            // Default RX Enabled -> \RE = 0 & DE = 0
            gpiob.bsrr.write(|w| w.br15().set_bit().br14().set_bit());
            // Disable emitter | Enable receiver
            gpiob.bsrr.write(|w| w.br15().set_bit());
            // Configure PA9/PA10 Alternate Function 1 -> USART1
            gpioa.ospeedr.write(|w| {
                w.ospeedr9().high_speed().ospeedr10().high_speed()
            });
            gpioa.pupdr.write(
                |w| w.pupdr9().pull_up().pupdr10().pull_up(),
            );
            gpioa.afrh.write(|w| w.afrh9().af1().afrh10().af1());
            gpioa.moder.write(
                |w| w.moder9().alternate().moder10().alternate(),
            );
            gpioa.otyper.write(
                |w| w.ot9().push_pull().ot10().push_pull(),
            );

            // Configure UART : Word length
            uart.cr1.modify(|_, w| w.m()._8bits());
            // Configure UART : Parity
            uart.cr1.modify(|_, w| w.pce().disabled());
            // Configure UART : Transfert Direction - Oversampling - RX Interrupt
            uart.cr1.modify(|_, w| {
                w.te()
                    .enabled()
                    .re()
                    .enabled()
                    .over8()
                    .over8()
                    .rxneie()
                    .enabled()
            });
            // Configure UART : 1 stop bit
            uart.cr2.modify(|_, w| w.stop()._1stop());

            // Configure UART : disable hardware flow control - Overrun interrupt
            uart.cr3.write(|w| {
                w.rtse()
                    .disabled()
                    .ctse()
                    .disabled()
                    .ctsie()
                    .disabled()
                    .ovrdis()
                    .disabled()
            });
            // Configure UART : baudrate
            uart.brr.write(|w| {
                w.div_fraction().bits(
                    (FREQUENCY / (baudrate / 2)) as u8 & 0x0F,
                )
            });
            uart.brr.write(|w| {
                w.div_mantissa().bits(
                    ((FREQUENCY / (baudrate / 2)) >> 4) as u16,
                )
            });
            // Configure UART : Asynchronous mode
            uart.cr2.modify(
                |_, w| w.linen().disabled().clken().disabled(),
            );
            // UART1 enabled
            uart.cr1.modify(|_, w| w.ue().enabled());
        });
        unsafe {
            RECV_CB = Some(extend_lifetime(&mut f));
        }
    }

    pub fn setup_debug(baudrate: u32) {
        cortex_m::interrupt::free(|cs| {
            let rcc = RCC.borrow(cs);
            let gpiob = GPIOB.borrow(cs);
            let uart = UART3.borrow(cs);

            // Enable GPIOB Clock
            rcc.ahbenr.modify(|_, w| w.iopben().enabled());
            // Enable USART3 Clock
            rcc.apb1enr.modify(|_, w| w.usart3en().enabled());
            // Configure PB10/PB11 Alternate Function 1 -> USART3
            gpiob.ospeedr.modify(|_, w| {
                w.ospeedr10().high_speed().ospeedr11().high_speed()
            });
            gpiob.pupdr.modify(
                |_, w| w.pupdr10().pull_up().pupdr11().pull_up(),
            );
            gpiob.afrh.modify(|_, w| w.afrh10().af4().afrh11().af4());
            gpiob.moder.modify(|_, w| {
                w.moder10().alternate().moder11().alternate()
            });
            gpiob.otyper.modify(
                |_, w| w.ot10().push_pull().ot11().push_pull(),
            );

            // Configure UART : Word length
            uart.cr1.modify(|_, w| w.m()._8bits());
            // Configure UART : Parity
            uart.cr1.modify(|_, w| w.pce().disabled());
            // Configure UART : Transfert Direction - Oversampling - RX Interrupt
            uart.cr1.modify(|_, w| {
                w.te()
                    .enabled()
                    .re()
                    .enabled()
                    .over8()
                    .over8()
                    .rxneie()
                    .enabled()
            });
            // Configure UART : 1 stop bit
            uart.cr2.modify(|_, w| w.stop()._1stop());

            // Configure UART : disable hardware flow control - Overrun interrupt
            uart.cr3.write(|w| {
                w.rtse()
                    .disabled()
                    .ctse()
                    .disabled()
                    .ctsie()
                    .disabled()
                    .ovrdis()
                    .disabled()
            });
            // Configure UART : baudrate
            uart.brr.write(|w| {
                w.div_fraction().bits(
                    (FREQUENCY / (baudrate / 2)) as u8 & 0x0F,
                )
            });
            uart.brr.write(|w| {
                w.div_mantissa().bits(
                    ((FREQUENCY / (baudrate / 2)) >> 4) as u16,
                )
            });
            // Configure UART3 : Asynchronous mode
            uart.cr2.modify(
                |_, w| w.linen().disabled().clken().disabled(),
            );
            // UART3 enabled
            uart.cr1.modify(|_, w| w.ue().enabled());
        });
    }

    /// Send a byte to the UART when it's ready.
    ///
    /// *Beware, this function will block until the UART is ready to send.*
    ///
    /// # Arguments
    ///
    /// * `byte` - The u8 byte to send.
    pub fn debug_send_when_ready(byte: u8) {
        cortex_m::interrupt::free(|cs| {
            let uart3 = UART3.borrow(cs);
            while !debug_transmit_complete(cs) {}
            uart3.tdr.write(|w| w.tdr().bits(byte as u16));
        })
    }

    fn debug_transmit_complete(cs: &cortex_m::interrupt::CriticalSection) -> bool {
        let uart3 = UART3.borrow(cs);
        if uart3.isr.read().tc().bit_is_set() {
            uart3.icr.write(|w| w.tccf().clear_bit());
            true
        } else {
            false
        }
    }

    /// Enable the Uart Interruption
    ///
    /// The callback passed to the `setup` function may now be called.
    pub fn enable_interrupt() {
        cortex_m::interrupt::free(|cs| {
            let nvic = NVIC.borrow(cs);
            nvic.enable(Interrupt::USART1);
            nvic.clear_pending(Interrupt::USART1);
        });
    }

    static mut RECV_CB: Option<&'static mut FnMut(u8)> = None;

    pub fn receive_callback(cs: &cortex_m::interrupt::CriticalSection) {
        let uart = UART1.borrow(cs);
        unsafe {
            DATA_UART1 = uart.rdr.read().rdr().bits();
        }
        unsafe {
            if let Some(ref mut cb) = RECV_CB {
                cb(DATA_UART1 as u8);
            }
        }
    }
    /// Send a byte to the UART when it's ready.
    ///
    /// *Beware, this function will block until the UART is ready to send.*
    ///
    /// # Arguments
    ///
    /// * `byte` - The u8 byte to send.
    pub fn send_when_ready(byte: u8) {
        cortex_m::interrupt::free(|cs| {
            let gpiob = GPIOB.borrow(cs);
            let uart1 = UART1.borrow(cs);
            // TX Enabled -> \RE = 1 & DE = 1
            gpiob.bsrr.write(|w| w.bs15().set_bit().bs14().set_bit());
            while !transmit_complete(cs) {}
            uart1.tdr.write(|w| w.tdr().bits(byte as u16));
        })
    }

    fn transmit_complete(cs: &cortex_m::interrupt::CriticalSection) -> bool {
        let uart1 = UART1.borrow(cs);
        if uart1.isr.read().tc().bit_is_set() {
            uart1.icr.write(|w| w.tccf().clear_bit());
            true
        } else {
            false
        }
    }

    pub fn receive() {
        cortex_m::interrupt::disable();

        cortex_m::interrupt::free(|cs| {
            let uart = UART1.borrow(cs);
            if uart.isr.read().rxne().bit_is_set() {
                receive_callback(cs);
            }
        });

        unsafe {
            cortex_m::interrupt::enable();
        }
    }

    unsafe fn extend_lifetime<'a>(f: &'a mut FnMut(u8)) -> &'static mut FnMut(u8) {
        core::mem::transmute::<&'a mut FnMut(u8), &'static mut FnMut(u8)>(f)
    }
}
#[cfg(target_arch = "arm")]
interrupt!(USART1, hard::receive);

#[cfg(not(target_arch = "arm"))]
mod soft {
    /// Setup the physical communication with the bus
    ///
    /// # Arguments
    ///
    /// * `baudrate` - A u32 specifying the communication baudrate
    /// * `f` - A `FnMut(u8)` reception callback - *WARNING: it will be called inside the interruption!*
    pub fn setup<F>(_baudrate: u32, mut _f: F)
    where
        F: FnMut(u8),
    {
    }
    /// Enable the Uart Interruption
    ///
    /// The callback passed to the `setup` function may now be called.
    pub fn enable_interrupt() {}
    /// Send a byte to the UART when it's ready.
    ///
    /// *Beware, this function will block until the UART is ready to send.*
    ///
    /// # Arguments
    ///
    /// * `byte` - The u8 byte to send.
    pub fn send_when_ready(_byte: u8) {}

    pub fn setup_debug(_baudrate: u32) {}
    pub fn debug_send_when_ready(byte: u8) {
        print!("{}", byte as char);
    }
}

#[cfg(target_arch = "arm")]
pub use self::hard::{setup, enable_interrupt, send_when_ready, setup_debug, debug_send_when_ready};
#[cfg(not(target_arch = "arm"))]
pub use self::soft::{setup, enable_interrupt, send_when_ready, setup_debug, debug_send_when_ready};
