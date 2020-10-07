// Don't link the Rust standard library
#![no_std]
// Disable all Rust-level entry points
#![no_main]
// We use our own custom test runner because we can't depend on `std`
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod serial;
mod vga_buffer;

/// The possible exit codes for exiting from QEMU.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

// Exit from QEMU, if running in it.
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        // QEMU's `isa-debug-exit` device is mapped to port `0xf4` in Cargo.toml.
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

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

    /// Marks a function as being a test function. This enables things like
    /// automatically printing the function's name when it is ran, for example.
    pub trait Testable {
        /// Run the `Testable`.
        fn run(&self) -> ();
    }

    /// Makes any `Fn()` `Testable`, which allows us to print its name to serial,
    /// run it, and then print the `[ok]` status if nothing went wrong.
    impl<T> Testable for T
    where
        T: Fn(),
    {
        fn run(&self) -> () {
            serial_print!("{}...\t", core::any::type_name::<T>());
            self();
            serial_println!("[ok]");
        }
    }

    /// Custom test runner since we can't depend on the `std` crate.
    pub fn test_runner(tests: &[&dyn Testable]) {
        serial_println!("Running {} tests", tests.len());

        for test in tests {
            test.run();
        }

        exit_qemu(QemuExitCode::Success);
    }

    /// This function is called on panic **when in test mode**.
    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        serial_println!("[failed]\n");
        serial_println!("Error: {}\n", info);
        exit_qemu(QemuExitCode::Failed);
        loop {}
    }

    #[test_case]
    fn trivial_assertion() {
        assert_eq!(1, 1);
    }
}
