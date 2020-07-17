#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(nokos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use nokos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    nokos::init();

    let ptr = 0xdeadbeaf as *mut u32;
    unsafe { *ptr = 42; }

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

