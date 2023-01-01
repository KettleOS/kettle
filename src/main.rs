#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate x86_64 as x86_64_util;

mod panic;
mod x86_64;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
	// initialization
	x86_64::gdt::init();
	x86_64::interrupts::init_interrupts();
	
	loop {}
}
