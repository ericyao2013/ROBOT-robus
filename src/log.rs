use alloc;
use core;

use physical;

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
