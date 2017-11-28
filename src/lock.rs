#[cfg(target_arch = "arm")]
use cortex_m;

pub fn take() {
    #[cfg(target_arch = "arm")] cortex_m::interrupt::disable();
}
pub fn release() {
    #[cfg(target_arch = "arm")]
    unsafe {
        cortex_m::interrupt::enable();
    }
}
