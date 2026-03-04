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

static HELLO: &[u8] = b"Hello World, how is it going?";

/// Program entry point
/// should not return hence !
/// as this function is not called by some other function
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// VGA Notes
// there is some buffer for VGA text
// seems you write 2 bytes, the first
// represents the ascii character
// and the second is the color / attribute
