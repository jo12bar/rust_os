#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rust_os::{exit_qemu, serial_print, serial_println, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();

    // If we get here, then the previous line didn't panic. So, we fail the test.
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);

    loop {}
}

/// This function should cause a panic.
fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

/// A custom panic handler that treats panics as successes.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
