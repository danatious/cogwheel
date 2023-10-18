#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cogwheel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;
use cogwheel::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	cogwheel::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	cogwheel::test_panic_handler(info)
}


async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}


extern crate alloc;
use cogwheel::task::{Task, keyboard};
use cogwheel::task::executor::Executor;

entry_point!(kernel_main);
pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
	use cogwheel::allocator;
	use cogwheel::memory::{self, BootInfoFrameAllocator};

	println!("Cogwheel starting...");

	cogwheel::init();

	let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
	let mut mapper = unsafe { memory::init(phys_mem_offset) };
	let mut frame_allocator = unsafe {
		BootInfoFrameAllocator::init(&boot_info.memory_map)
	};

	allocator::init_heap(&mut mapper, &mut frame_allocator)
		.expect("heap initialization failed");
    
	
	#[cfg(test)]
	test_main();

    // Test PCI scan
    use cogwheel::pci;
    pci::scan_pci();

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}