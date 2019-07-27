#![no_std]  // Don't link the Rust standard library.
#![no_main] // Disable all Rust-level entry points.

#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use rust_os::println;

entry_point!(kernel_main);

/// This function is the entry point, since the linker looks for a function named
/// `_start` by default. Currently, it prints "Hello World!" to the VGA buffer.
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Hello World{}", "!");

    rust_os::init();

    let mut mapper = unsafe { memory::init(boot_info.physical_memory_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // Map a previously unmapped page
    let page = Page::containing_address(VirtAddr::new(0xdeadbead000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // Write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    // Test allocating a value on the heap.
    let x = Box::new(4);

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
