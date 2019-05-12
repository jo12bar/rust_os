#![no_std]  // Don't link the Rust standard library.
#![no_main] // Disable all Rust-level entry points.

mod vga_buffer;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// This function is the entry point, since the linker looks for a function named
/// `_start` by default. Currently, it prints "Hello World!" to the VGA buffer.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Some panic message");

    loop {}
}
