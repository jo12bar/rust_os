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
    use rust_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Hello World!");

    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // Map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // Write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0xf021_f077_f065_f04e) };

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
