#![no_std]
#![no_main]

// Features
#![feature(format_args_nl)]

// Lints
#![forbid(unsafe_op_in_unsafe_fn)]

use platform::aarch64::UART0;

mod cpu;
mod panic;
mod platform;
mod console;
mod io;
mod brand;

/// The main kernel entrypoint. This function is non-mangled for debug purposes; however, it is incompatible with FFI.
///
/// It is ill-advised to call this function directly. Instead, implement an [_init_rt](cpu::boot::_init_rt) wrapper and call [kernel_main] from there.
///
/// # Safety
/// You **must not** call this function outside of an [_init_rt](cpu::boot::_init_rt) wrapper. Calling this function after it has already been called will result in unexplainable crashes and is considered Undefined Behavior.
#[no_mangle]
pub unsafe fn kernel_main() {
	println!("Hello world! UART0 is 0x{UART0:X}.");
	panic!("Exiting from the kernel early. Bye bye!");
}
