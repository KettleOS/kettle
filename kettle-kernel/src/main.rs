#![no_std]
#![no_main]

// Features
#![feature(format_args_nl)]

// Lints
#![forbid(unsafe_op_in_unsafe_fn)]

use core::sync::atomic::AtomicBool;

use brand::*;

mod cpu;
mod panic;
mod platform;
mod console;
mod io;
mod brand;
mod sync;
mod driver;

static KERNEL_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Kernel and driver initialization happens here.
///
/// # Safety
/// You **must not** call this function outside of an [_init_rt](cpu::boot::_init_rt) wrapper. Calling this function after it has already been called will result in unexplainable crashes and is considered Undefined Behavior.
unsafe fn kernel_init() {}

/// The main kernel entrypoint.
///
/// It is ill-advised to call this function directly. Instead, implement an [_init_rt](cpu::boot::_init_rt) wrapper and call [kernel_main] from there.
///
/// # Safety
/// You **must not** call this function outside of an [_init_rt](cpu::boot::_init_rt) wrapper. Calling this function after it has already been called will result in unexplainable crashes and is considered Undefined Behavior.
unsafe fn kernel_main() {
	print_kernel_brand();

	panic!("Exiting from the kernel early. Bye bye!");
}

/// Prints the kernel brand.
fn print_kernel_brand() {
	if KERNEL_INITIALIZED.load(core::sync::atomic::Ordering::Relaxed) {
		println!("{KERNEL_BRAND} v{KERNEL_VERSION}");
	}
}
