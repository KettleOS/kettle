#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod logger;
mod interrupts;
mod panic;
mod gdt;

use core::borrow::BorrowMut;
use bootloader::{BootInfo, entry_point};
use crate::logger::{BACKGROUND, ERR_COLOR, FOREGROUND, init_logger};

entry_point!(kernel_main);

#[no_mangle]
pub fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
	let boot_info: &'static mut BootInfo = (*boot_info).borrow_mut();
	// pray to god this doesn't fail
	if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
		init_logger(framebuffer, FOREGROUND, BACKGROUND, ERR_COLOR);
	} else {
		loop {}
	}
	
	println!("Hello world!");
	
	x86_64::instructions::interrupts::int3();
	
	// init shit
	init();
	
	println!("Didn't crash");
	
	loop {}
}

fn init() {
	gdt::init();
	interrupts::init_interrupts();
}
