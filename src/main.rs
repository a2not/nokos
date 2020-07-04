#![no_std] // force not to link the Rust standard library
#![no_main] // disable all Rust-level entry points

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}


#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

mod vga_buffer;

#[no_mangle] // force not to mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    
    #[cfg(test)]
    test_main();

    loop {}
}

