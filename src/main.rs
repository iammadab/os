// the goal here is to create a freestanding binary
// that does not depend on any of the os features
// as we are trying to create our own operating system
#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

/// this is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// Program entry point
/// should not return hence !
/// as this function is not called by some other function
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    loop {}
}

// VGA Notes
// there is some buffer for VGA text
// seems you write 2 bytes, the first
// represents the ascii character
// and the second is the color / attribute
