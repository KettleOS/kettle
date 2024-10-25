#![no_std]
#![no_main]

mod cpu;
mod panic;
mod platform;

#[no_mangle]
pub extern "C" fn kernel_main() {}
