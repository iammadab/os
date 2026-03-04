// the goal here is to create a freestanding binary
// that does not depend on any of the os features
// as we are trying to create our own operating system
#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// this is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Program entry point
/// should not return hence !
/// as this function is not called by some other function
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}
