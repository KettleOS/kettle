#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod logger;
mod interrupts;
mod panic;

use bootloader::{BootInfo, entry_point};
use crate::interrupts::init_idt;
use crate::logger::{BACKGROUND, ERR_COLOR, FOREGROUND, init_logger};

entry_point!(kernel_main);

pub fn kernel_main(info: &'static mut BootInfo) -> ! {
	if let Some(framebuffer) = info.framebuffer.as_mut() {
		init_logger(framebuffer, FOREGROUND, BACKGROUND, ERR_COLOR);
	} // pray to god this doesn't fail
	
	println!("Hello world!");
	
	// initialize idt
	init_idt();
	
	// cause a pagefault
	unsafe {
		*(0xDEADBEEF as *mut usize) = 42;
	}
	
	println!("Didn't crash");
	
	loop {}
}
