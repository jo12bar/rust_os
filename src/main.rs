#![no_std]  // Don't link the Rust standard library.
#![no_main] // Disable all Rust-level entry points.

#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use rust_os::println;

entry_point!(kernel_main);

/// This function is the entry point, since the linker looks for a function named
/// `_start` by default. Currently, it prints "Hello World!" to the VGA buffer.
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::memory;
    use x86_64::{structures::paging::MapperAllSizes, VirtAddr};

    println!("Hello World{}", "!");

    rust_os::init();

    let mapper = unsafe { memory::init(boot_info.physical_memory_offset) };

    let addresses = [
        // The identity-mapped vga buffer page
        0xb8000,
        // Some code page
        0x20010a,
        // Some stack page
        0x57ac_001f_fe48,
        // Virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();

    println!("It didn't crash! Yay!");
    rust_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}
