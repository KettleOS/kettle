#![no_std]
#![no_main]

mod logger;

use core::fmt::Write;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use crate::logger::Logger;

static HELLO: &'static str = "Hello world!";

entry_point!(kernel_main);

pub fn kernel_main(info: &'static mut BootInfo) -> ! {
	if let Some(framebuffer) = info.framebuffer.as_mut() {
		let mut logger = Logger::new(framebuffer, (255, 255, 255));
		logger.write_str(HELLO);
	}
	
	loop {}
}

// called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}
