#![no_std]
#![no_main]

// Features
#![feature(format_args_nl)]

// Lints
#![forbid(unsafe_op_in_unsafe_fn)]

use core::sync::atomic::AtomicBool;

use brand::*;
use driver::{KernelDriverDescriptor, DRIVER_MANAGER};
use error::KernelResult;

mod cpu;
mod panic;
mod platform;
mod console;
mod io;
mod brand;
mod sync;
mod driver;
mod error;
mod fmt;

static PRINT_DRIVER_INITIALIZED: AtomicBool = AtomicBool::new(false);
static KERNEL_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Kernel and driver initialization happens here.
///
/// # Safety
/// You **must not** call this function outside of an [_init_rt](cpu::boot::_init_rt) wrapper. Calling this function after it has already been called will result in unexplainable crashes and is considered Undefined Behavior.
///
/// Calling this function twice is only checked in debug builds.
unsafe fn kernel_init() -> KernelResult<()> {
	debug_assert!(!KERNEL_INITIALIZED.load(core::sync::atomic::Ordering::Relaxed), "`kernel_init` must not be called twice!");

	#[cfg(feature = "platform_virt")]
	{
		let driver = KernelDriverDescriptor::new(
			&platform::aarch64::driver::UART_DRIVER,
			None,
		);
		let driver = KernelDriverDescriptor::new();
		DRIVER_MANAGER.register(driver)?;
	}

	PRINT_DRIVER_INITIALIZED.store(true, core::sync::atomic::Ordering::Relaxed);
	KERNEL_INITIALIZED.store(true, core::sync::atomic::Ordering::Relaxed);

	debug_assert!(KERNEL_INITIALIZED.load(core::sync::atomic::Ordering::Relaxed) && PRINT_DRIVER_INITIALIZED.load(core::sync::atomic::Ordering::Relaxed), "Invalid state: kernel and print driver should be initialized by the end of `kernel_init`.");

	Ok(())
}

/// The main kernel entrypoint.
///
/// It is ill-advised to call this function directly. Instead, implement an [_init_rt](cpu::boot::_init_rt) wrapper and call [kernel_main] from there.
///
/// # Safety
/// You **must not** call this function outside of an [_init_rt](cpu::boot::_init_rt) wrapper. Calling this function after it has already been called will result in unexplainable crashes and is considered Undefined Behavior.
unsafe fn kernel_main() {
	if !KERNEL_INITIALIZED.load(core::sync::atomic::Ordering::Relaxed) {
		// We should be initialized by now. Panic!
		unreachable!("`kernel_main` called before kernel was initialized. This should never happen! For kernel developers: rectify your implementation of `_init_rt`.");
	}

	print_kernel_brand();

	unimplemented!("Exiting from the kernel early. Bye bye!");
}

/// Prints the kernel brand.
///
/// # Panics
/// Panics if called before [kernel_init] was called.
fn print_kernel_brand() {
	check_kernel_initialized();
	println!("{KERNEL_BRAND} v{KERNEL_VERSION}");
}

/// Checks if the kernel is initialized.
///
/// # Panics
/// Panics if the kernel is not initialized.
#[inline(always)]
fn check_kernel_initialized() {
	if !KERNEL_INITIALIZED.load(core::sync::atomic::Ordering::Relaxed) {
		panic!("Attempted to call a function before the kernel was initialized.");
	}
}
