#![no_std] // force not to link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

mod vga_buffer;

#[no_mangle] // force not to mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Some panic message");
    
    // loop {}
}

