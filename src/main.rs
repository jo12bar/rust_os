// Don't link the Rust standard library
#![no_std]
// Disable all Rust-level entry points
#![no_main]
// We use our own custom test runner because we can't depend on `std`
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;

/// Main kernel entry point. Uses the Linux convention of being called `_start`.
/// That also means that we have to avoid mangling the function's name.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    // If this kernal was started via `cargo test`, then run all the tests.
    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic **when _not_ in test mode**.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
mod tests {
    use super::*;

    /// This function is called on panic **when in test mode**.
    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        rust_os::test_panic_handler(info);
    }

    #[test_case]
    fn trivial_assertion() {
        assert_eq!(1, 1);
    }
}
