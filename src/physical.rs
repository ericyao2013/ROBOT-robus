//! Handles the physical communication with the bus.
//!
//! This module handles the physical aspect of the communication with the bus. In particular, it correctly sets the UART communication and the associated GPIOs.

#[cfg(target_arch = "arm")]
mod hard {
    use core;
    use robus_core;
    use recv_buf;
    use Message;

    use hal::prelude::*;

    struct Interface<F, USART, TX, RX, TIMER, DE, RE, PTPA, PTPB>
    where
        TX: TxPin<USART>,
        RX: RxPin<USART>,
        F: FnMut(u8),
        DE: Output<PushPull>,
        RE: Output<PushPull>,
        PTPA: Input<PullUp>,
        PTPB: Input<PullUp>,
    {
        tx: TX,
        rx: RX,
        callback: F,
        timer: TIMER,
        de: DE,
        re: RE,
        ptpa: PTPA,
        ptpb: PTPB,
        baudrate: u32,
    }
    impl Interface {
        /// Setup the physical communication with the bus
        ///
        /// # Arguments
        ///
        /// * `baudrate` - A u32 specifying the communication baudrate
        /// * `f` - A `FnMut(u8)` reception callback - *WARNING: it will be called inside the interruption!*
        pub fn setup<F, USART, TX, RX, TIMER, DE, RE, PTPA, PTPB>(uart: USART, re: RE, de: DE, ptpa: PTPA, ptpb: PTPB, timer: Timer, mut callback: F, baudrate: u32) -> Interface
        where
            TX: TxPin<USART>,
            RX: RxPin<USART>,
            F: FnMut(u8),
            DE: Output<PushPull>,
            RE: Output<PushPull>,
            PTPA: Input<PullUp>,
            PTPB: Input<PullUp>,
        {
            // timer need to be not continue timer, we should have a type for that
            /// manage timer depending on baudrate
            timeout_duration = 1/(baudrate * 9);
            self.timer.setup(self.timeout_duration);

            /// Put Robus driver in RX enabled mode (default mode) -> \RE = 0 & DE = 0
            de.set_low();
            re.set_low();
            let (mut tx, mut rx) = serial.split();

            Interface {
                tx,
                rx,
                callback,
                timer,
                de,
                re,
                ptpa,
                ptpb,
                timeout_duration,
                baudrate,
            }
        }
        /// Change the robus main baudrate
        ///
        /// # Arguments
        ///
        /// * `baudrate` - A u32 specifying the communication baudrate
        pub fn set_baudrate(self, uart: USART, baudrate: u32) {
                self.uart = uart;
                self.baudrate = baudrate;
                self.timeout_duration = 1/(self.baudrate * 9);
                self.timer.setup(self.timeout_duration);
        }

        fn tx_enable(self) {
            self.re.set_high()
        }
        fn tx_disable(self) {
            self.re.set_low()
        }
        fn rx_enable(self) {
            self.de.set_low()
        }
        fn rx_disable(self) {
            self.de.set_high()
        }
        /// Send a Message to the UART.
        ///
        /// *Beware, this function will block until the Message is send.*
        ///
        /// # Arguments
        ///
        /// * `Msg` - The Message to send.
        pub fn send(self, msg: &mut Message) {
            self.tx_enable();
            self.rx_disable();
            for byte in msg.to_bytes() {
                block!(self.tx.write(byte)).ok();
            }
            self.tx_disable();
            self.rx_enable();
            self.timer.start();
        }

        pub fn receive(self) {
                // we receive something, start timeout
                self.timer.start();

                // get received u8
                let uart_val = block!(rx.read()).unwrap();
                self.callback(uart_val as u8);
        }

        /// Setup the timeout Timer
        ///
        /// The timer is used to trigger timeout event and flush the reception buffer if we read corrupted data.
        pub fn timeout() {
                // if we manage TX_lock in this level we could avoid this static
                // TX_LOCK release
                unsafe {
                    robus_core::TX_LOCK = false;
                }
                // flush message buffer
                recv_buf::flush();
        }
    }
}

#[cfg(not(target_arch = "arm"))]
mod soft {
    /// Change the robus main baudrate
    ///
    /// # Arguments
    ///
    /// * `baudrate` - A u32 specifying the communication baudrate
    pub fn set_baudrate(_baudrate: u32) {}
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

}

#[cfg(target_arch = "arm")]
pub use self::hard::*;
#[cfg(not(target_arch = "arm"))]
pub use self::soft::*;
