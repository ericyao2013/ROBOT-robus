use alloc;
use core;

use physical;

/// Uart Logger implementation
///
/// *It should only be used through the `log` macro.*
/// TODO: Could we find a way to hide it?
pub static mut LOGGER: UartLogger = UartLogger {};

pub struct UartLogger {}
impl core::fmt::Write for UartLogger {
    fn write_str(&mut self, s: &str) -> Result<(), alloc::fmt::Error> {
        for &b in s.as_bytes() {
            physical::debug_send_when_ready(b);
        }
        Ok(())
    }
}

#[macro_export]
/// Log macro that sends a fmt to the debug UART.
///
/// *You must called `robus::init()` before using the macro!*
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate robus;
///
/// use std::fmt::Write;
///
/// fn main() {
///    robus::init();
///
///    let x = 42;
///    log!("x: {:?}", x);
/// }
/// ```
macro_rules! log {
    ($fmt: expr) => ({
        let mut w = unsafe { &mut $crate::LOGGER };
        writeln!(&mut w, $fmt).unwrap();
    });
    ($fmt: expr, $($arg: tt)*) => ({
        let mut w = unsafe { &mut $crate::LOGGER };
        writeln!(&mut w, $fmt, $($arg)*).unwrap();
    });
}
