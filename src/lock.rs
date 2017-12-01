#[cfg(target_arch = "arm")]
use cortex_m;

// TODO: Implement the lock using a more friendly API.
// Maybe something like:
// ```
// let lock = Lock::new();
// let res = lock.use(|| {
//    //  my compution in a locked section.
//    12 * 32
// });
// ```

pub fn take() {
    #[cfg(target_arch = "arm")] cortex_m::interrupt::disable();
}
pub fn release() {
    #[cfg(target_arch = "arm")]
    unsafe {
        cortex_m::interrupt::enable();
    }
}
