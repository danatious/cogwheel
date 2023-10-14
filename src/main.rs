#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cogwheel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use cogwheel::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cogwheel::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    cogwheel::init();
    
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}