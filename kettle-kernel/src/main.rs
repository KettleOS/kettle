#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

#[cfg(target_arch = "x86_64")]
extern crate x86_64 as x86_64_core;

mod panic;
#[cfg(target_arch = "x86_64")]
mod x86_64;

#[no_mangle]
#[cfg(target_arch = "x86_64")]
pub extern "C" fn kernel_main() -> ! {
	x86_64_core::instructions::interrupts::int3();
	
	// init shit
	init();
	
	loop {}
}

fn init() {
	#[cfg(target_arch = "x86_64")]
	{
		x86_64::gdt::init();
		x86_64::interrupts::init_interrupts();
	}
}
