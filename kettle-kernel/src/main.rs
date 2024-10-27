#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

mod cpu;
mod panic;
mod platform;

/// The main kernel entrypoint. This function is non-mangled for debug purposes; however, it is incompatible with FFI.
///
/// It is ill-advised to call this function directly. Instead, implement an [_init_rt](cpu::boot::_init_rt) wrapper and call [kernel_main] from there.
#[no_mangle]
pub unsafe fn kernel_main() {}
