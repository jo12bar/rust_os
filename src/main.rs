// Don't link the Rust standard library
#![no_std]
// Disable all Rust-level entry points
#![no_main]
// We use our own custom test runner because we can't depend on `std`
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rust_os::println;

entry_point!(kernel_main);

/// Main kernel entry point. Uses the Linux convention of being called `_start`.
/// That also means that we have to avoid mangling the function's name.
pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::memory::translate_addr;
    use x86_64::VirtAddr;

    println!("Hello World!");

    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let addresses = [
        // The identity-mapped vga buffer page
        0xb8000,
        // Some code page
        0x201008,
        // Some stack page
        0x0100_0020_1a10,
        // Virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe { translate_addr(virt, phys_mem_offset) };
        println!("{:?} -> {:?}", virt, phys);
    }

    // If this kernel was started via `cargo test`, then run all the tests.
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    rust_os::hlt_loop();
}

/// This function is called on panic **when _not_ in test mode**.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
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
