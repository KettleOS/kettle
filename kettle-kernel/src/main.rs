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

static KERNEL_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// The main kernel entrypoint. This function is non-mangled for debug purposes; however, it is incompatible with FFI.
///
/// It is ill-advised to call this function directly. Instead, implement an [_init_rt](cpu::boot::_init_rt) wrapper and call [kernel_main] from there.
///
/// # Safety
/// You **must not** call this function outside of an [_init_rt](cpu::boot::_init_rt) wrapper. Calling this function after it has already been called will result in unexplainable crashes and is considered Undefined Behavior.
#[no_mangle]
pub unsafe fn kernel_main() {
	// SAFETY:
	// This function is only called once.
	unsafe { kernel_init() };

	print_kernel_brand();

	panic!("Exiting from the kernel early. Bye bye!");
}

/// Prints the kernel brand.
fn print_kernel_brand() {
	if KERNEL_INITIALIZED.load(core::sync::atomic::Ordering::Relaxed) {
		println!("{KERNEL_BRAND} v{KERNEL_VERSION}");
	}
}

/// Kernel and driver initialization happens here.
///
/// # Safety
/// You **must not** call this function twice. This function is originally called in [kernel_main] and should only be called there.
unsafe fn kernel_init() {}
