// Don't link the Rust standard library
#![no_std]
// Disable all Rust-level entry points
#![no_main]
// We use our own custom test runner because we can't depend on `std`
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rust_os::{
    println,
    task::{executor::Executor, keyboard, Task},
};

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

entry_point!(kernel_main);

/// Main kernel entry point. Uses the Linux convention of being called `_start`.
/// That also means that we have to avoid mangling the function's name.
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::{
        allocator,
        memory::{self, BootInfoFrameAllocator},
    };
    use x86_64::VirtAddr;

    println!("Hello World!");

    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // Allocate a number on the heap
    let x = Box::new(41);
    println!("x = {:?} at heap address {0:p}", x);

    // Create a dynamically-sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at heap address {:p}", vec.as_slice());

    // Create a reference-counted vector (will be freed when count reaches 0)
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "reference_counted = {0:?} at heap address {0:p}\ncloned_reference = {1:?} at heap address {1:p}",
        &reference_counted, &cloned_reference
    );
    println!(
        "Current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

    // If this kernel was started via `cargo test`, then run all the tests.
    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
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
