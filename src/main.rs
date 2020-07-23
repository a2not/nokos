#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(nokos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use nokos::println;
use bootloader::{BootInfo, entry_point};
use alloc::boxed::Box;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::VirtAddr;
    use nokos::allocator;
    use nokos::memory::{self, BootInfoFrameAllocator};
    
    println!("Hello World{}", "!");
    nokos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe{
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
    
    let x = Box::new(41);

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    nokos::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    nokos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    nokos::test_panic_handler(info)
}


