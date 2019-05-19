#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod serial;
pub mod interrupts;
pub mod vga_buffer;

use core::panic::PanicInfo;

/// Initialize everything.
pub fn init() {
    interrupts::init_idt();
}

/// Custom test runner. Currently requires nightly rust.
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

/// Handler for when an assert in a test fails and panics.
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// Entry point for `cargo xtest`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

/// Custom panic handler, specifically for tests.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

/// Possible QEMU exit codes for use during testing and debugging. See the `Cargo.toml` for
/// where the custom successful exit code is set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/// Exits QEMU by writing some exit code to Port `0xf4`. For use during debugging and testing.
/// Port `0xf4` is bound in the `Cargo.toml` to the special QEMU device "isa-debug-exit",
/// which causes QEMU to exit whenever some value is written to it (which is then returned
/// as the emulator's exit code).
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
